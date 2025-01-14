#[macro_use(stmt)]
extern crate cassandra;
use std::str::FromStr;
use cassandra::*;

#[derive(Debug,PartialEq,Clone,Copy)]
struct Basic {
    bln: bool,
    flt: f32,
    dbl: f64,
    i32: i32,
    i64: i64,
}

fn insert_into_basic(session: &mut Session, key: &str, basic: Basic) -> Result<CassResult, CassError> {

    let mut statement = stmt!("INSERT INTO examples.basic (key, bln, flt, dbl, i32, i64) VALUES (?, ?, ?, ?, ?, ?);");
    try!(statement.bind(0, key));
    try!(statement.bind(1, basic.bln));
    try!(statement.bind(2, basic.flt));
    try!(statement.bind(3, basic.dbl));
    try!(statement.bind(4, basic.i32));
    try!(statement.bind(5, basic.i64));
    session.execute(&statement).wait()
}

fn select_from_basic(session: &mut Session, key: &str) -> Result<Option<Basic>, CassError> {
    let mut statement = stmt!("SELECT * FROM examples.basic WHERE key = ?");
    try!(statement.bind_string(0, key));
    let result = try!(session.execute(&statement).wait());
    println!("Result: \n{:?}\n", result);
    match result.first_row() {
        None => Ok(None),
        Some(row) => {
            Ok(Some(Basic {
                bln: try!(row.get_col(1)),
                dbl: try!(row.get_col(2)),
                flt: try!(row.get_col(3)),
                i32: try!(row.get_col(4)),
                i64: try!(row.get_col(5)),
            }))
        }
    }
}

fn main() {

    let input = Basic {
        bln: true,
        flt: 0.001f32,
        dbl: 0.0002f64,
        i32: 1,
        i64: 2,
    };

    let contact_points = ContactPoints::from_str("127.0.0.1").unwrap();

    let mut cluster = Cluster::new();
    cluster.set_contact_points(contact_points).unwrap();
    cluster.set_load_balance_round_robin();

    match cluster.connect() {
        Ok(ref mut session) => {
            let ks_statement = &stmt!("CREATE KEYSPACE IF NOT EXISTS examples WITH replication = { \'class\': \
                                       \'SimpleStrategy\', \'replication_factor\': \'1\' };");

            let table_statement = &stmt!("CREATE TABLE IF NOT EXISTS examples.basic (key text, bln boolean, flt \
                                          float, dbl double, i32 int, i64 bigint, PRIMARY KEY (key));");

            session.execute(ks_statement).wait().unwrap();
            session.execute(table_statement).wait().unwrap();

            insert_into_basic(session, "test", input).unwrap();
            let output = select_from_basic(session, "test").unwrap().expect("no output from select");

            println!("{:?}", input);
            println!("{:?}", output);

            assert!(input == output);
        }
        err => println!("{:?}", err),
    }
}
