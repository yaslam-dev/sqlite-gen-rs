use std::mem;

use sqlite_loadable::table::{ConstraintOperator, VTab};
use sqlite_loadable::{api, table::VTabCursor, Result};
use sqlite_loadable::{define_table_function, prelude::*, BestIndexError};

static CREATE_SQL: &str = "CREATE TABLE x(value, start hidden, stop hidden, step hidden)";
enum Columns {
    Value,
    Start,
    Stop,
    Step,
}

fn column(idx: i32) -> Option<Columns> {
    match idx {
        0 => Some(Columns::Value),
        1 => Some(Columns::Start),
        2 => Some(Columns::Stop),
        3 => Some(Columns::Step),
        _ => None,
    }
}
#[repr(C)]
pub struct GeneratSeriesCursor {
    base: sqlite3_vtab_cursor,
    rowid: i64,
    value: i64,
    start: i64,
    stop: i64,
    step: i64,
}

impl GeneratSeriesCursor {
    fn new() -> GeneratSeriesCursor {
        let base: sqlite3_vtab_cursor = unsafe { mem::zeroed() };
        GeneratSeriesCursor {
            base,
            rowid: 0,
            value: 0,
            start: 0,
            stop: 0,
            step: 0,
        }
    }
}

impl VTabCursor for GeneratSeriesCursor {
    fn filter(
        &mut self,
        _idx_num: std::os::raw::c_int,
        _idx_str: Option<&str>,
        values: &[*mut sqlite3_value],
    ) -> Result<()> {
        self.start = api::value_int64(values.get(0).expect("start constraint is required"));
        self.stop = api::value_int64(values.get(1).expect("stop constraint is required"));
        self.step = api::value_int64(values.get(2).expect("step constraint is required"));
        self.value = self.start;

        Ok(())
    }

    fn next(&mut self) -> Result<()> {
        self.value += self.step;
        Ok(())
    }

    fn eof(&self) -> bool {
        self.value > self.stop
    }

    fn column(&self, ctx: *mut sqlite3_context, i: std::os::raw::c_int) -> Result<()> {
        match column(i) {
            Some(Columns::Value) => api::result_int64(ctx, self.value),
            Some(Columns::Start) => api::result_int64(ctx, self.start),
            Some(Columns::Stop) => api::result_int64(ctx, self.stop),
            Some(Columns::Step) => api::result_int64(ctx, self.step),

            None => (),
        }
        Ok(())
    }

    fn rowid(&self) -> Result<i64> {
        Ok(self.rowid)
    }
}

#[repr(C)]
pub struct GenerateSeriesTable {
    base: sqlite3_vtab,
}

impl<'vtab> VTab<'vtab> for GenerateSeriesTable {
    type Aux = ();
    type Cursor = GeneratSeriesCursor;

    fn connect(
        _db: *mut sqlite3,
        _aux: Option<&Self::Aux>,
        _args: sqlite_loadable::table::VTabArguments,
    ) -> Result<(String, GenerateSeriesTable)> {
        let base: sqlite3_vtab = unsafe { mem::zeroed() };
        let vtab: GenerateSeriesTable = GenerateSeriesTable { base };
        Ok((CREATE_SQL.to_owned(), vtab))
    }

    fn best_index(
        &self,
        mut info: sqlite_loadable::table::IndexInfo,
    ) -> core::result::Result<(), sqlite_loadable::BestIndexError> {
        let mut has_start = false;
        let mut has_stop = false;
        let mut has_step = false;
        for mut constraint in info.constraints() {
            match column(constraint.column_idx()) {
                Some(Columns::Start) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(1);
                        has_start = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                Some(Columns::Stop) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(2);
                        has_stop = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                Some(Columns::Step) => {
                    if constraint.usable() && constraint.op() == Some(ConstraintOperator::EQ) {
                        constraint.set_omit(true);
                        constraint.set_argv_index(3);
                        has_step = true;
                    } else {
                        return Err(BestIndexError::Constraint);
                    }
                }
                _ => todo!(),
            }
        }
        if !has_start || !has_stop || !has_step {
            return Err(BestIndexError::Error);
        }
        info.set_estimated_cost(100000.0);
        info.set_estimated_rows(100000);
        info.set_idxnum(1);

        Ok(())
    }

    fn open(&'vtab mut self) -> Result<GeneratSeriesCursor> {
        Ok(GeneratSeriesCursor::new())
    }

    fn create(
        db: *mut sqlite3,
        aux: Option<&Self::Aux>,
        args: sqlite_loadable::table::VTabArguments,
    ) -> Result<(String, Self)> {
        Self::connect(db, aux, args)
    }

    fn destroy(&self) -> Result<()> {
        Ok(())
    }
}

#[sqlite_entrypoint]
pub fn sqlite3_sqlitegenrs_init(db: *mut sqlite3) -> Result<()> {
    let _ = define_table_function::<GenerateSeriesTable>(db, "generate_series_rs", None);
    Ok(())
}
