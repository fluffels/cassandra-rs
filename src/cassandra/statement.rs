use std::ffi::CString;
// use decimal::d128;
use cassandra::collection::Set;
use cassandra::collection::Map;
use cassandra::collection::List;
use cassandra::error::CassError;
use cassandra::uuid::Uuid;
use cassandra::inet::Inet;
use cassandra::result::CassResult;
use cassandra::consistency::Consistency;
use cassandra::user_type::UserType;
use cassandra::batch::CustomPayload;
use cassandra::policy::retry::RetryPolicy;
use cassandra::tuple::Tuple;
use cassandra_sys::cass_true;
use cassandra_sys::cass_false;
use cassandra_sys::CassStatement as _Statement;
use cassandra_sys::cass_statement_new;
use cassandra_sys::cass_statement_free;
use cassandra_sys::cass_statement_add_key_index;
use cassandra_sys::cass_statement_set_keyspace;
use cassandra_sys::cass_statement_set_consistency;
use cassandra_sys::cass_statement_set_serial_consistency;
use cassandra_sys::cass_statement_set_paging_size;
use cassandra_sys::cass_statement_set_paging_state;
use cassandra_sys::cass_statement_bind_null;
use cassandra_sys::cass_statement_bind_null_by_name;
use cassandra_sys::cass_statement_bind_uint32;
use cassandra_sys::cass_statement_bind_uint32_by_name;
use cassandra_sys::cass_statement_bind_int32;
use cassandra_sys::cass_statement_bind_int32_by_name;
use cassandra_sys::cass_statement_bind_int16;
use cassandra_sys::cass_statement_bind_int16_by_name;
use cassandra_sys::cass_statement_bind_int8;
use cassandra_sys::cass_statement_bind_int8_by_name;
use cassandra_sys::cass_statement_bind_int64;
use cassandra_sys::cass_statement_bind_float;
use cassandra_sys::cass_statement_bind_double;
use cassandra_sys::cass_statement_bind_bool;
use cassandra_sys::cass_statement_bind_string;
use cassandra_sys::cass_statement_bind_bytes;
use cassandra_sys::cass_statement_bind_tuple;
use cassandra_sys::cass_statement_bind_tuple_by_name;
use cassandra_sys::cass_statement_bind_user_type;
use cassandra_sys::cass_statement_bind_user_type_by_name;
use cassandra_sys::cass_statement_bind_collection;
#[allow(unused_imports)]
use cassandra_sys::cass_statement_bind_decimal;
#[allow(unused_imports)]
use cassandra_sys::cass_statement_bind_decimal_by_name;
use cassandra_sys::cass_statement_bind_inet;
use cassandra_sys::cass_statement_bind_uuid;
use cassandra_sys::cass_statement_bind_int64_by_name;
use cassandra_sys::cass_statement_bind_float_by_name;
use cassandra_sys::cass_statement_bind_double_by_name;
use cassandra_sys::cass_statement_bind_bool_by_name;
use cassandra_sys::cass_statement_bind_string_by_name;
use cassandra_sys::cass_statement_bind_bytes_by_name;
use cassandra_sys::cass_statement_bind_collection_by_name;
use cassandra_sys::cass_statement_bind_inet_by_name;
use cassandra_sys::cass_statement_bind_uuid_by_name;
use cassandra_sys::cass_statement_set_custom_payload;
use cassandra_sys::cass_statement_set_paging_state_token;
use cassandra_sys::cass_statement_set_retry_policy;
use cassandra_sys::cass_statement_set_timestamp;
use cassandra::util::Protected;
///A statement object is an executable query. It represents either a regular
///(adhoc) statement or a prepared statement. It maintains the queries' parameter
///values along with query options (consistency level, paging state, etc.)
///
///<b>Note:</b> Parameters for regular queries are not supported by the binary protocol
///version 1.
pub struct Statement(*mut _Statement);

impl Protected<*mut _Statement> for Statement {
    fn inner(&self) -> *mut _Statement {
        self.0
    }
    fn build(inner: *mut _Statement) -> Self {
        Statement(inner)
    }
}

#[macro_export]
macro_rules! stmt {
    ( $( $x:expr ),*) => {
        {
            $(
        	let query = $x;
        	let param_count = query.matches("?").count();
        	let statement = Statement::new(query, param_count as u64);
            )*
            statement
        }
    };
}

// statement,
// 	key,
// 	i % 2 == 0,
// 	i as f32 / 2.0f32,
// 	i as f64 / 200.0,
// 	i as i32 * 10,
// 	i as i64 * 100);


impl Drop for Statement {
    ///Frees a statement instance. Statements can be immediately freed after
    ///being prepared, executed or added to a batch.
    fn drop(&mut self) {
        unsafe { self.free() }
    }
}

///All Rust types that can be bound to a cassandra statement
/// //FIXME not yet implemented
/// pub enum CassBindable {
///
/// }
///Any rust type that can have a default bind implementation
pub trait BindRustType<T> {
    ///binds a rust type to C* by index
    fn bind(&mut self, index: u64, value: T) -> Result<&mut Statement, CassError>;
    ///binds a rust type to C* by name
    fn bind_by_name(&mut self, col: &str, value: T) -> Result<&mut Statement, CassError>;
}

impl BindRustType<bool> for Statement {
    fn bind(&mut self, index: u64, value: bool) -> Result<&mut Self, CassError> {
        self.bind_bool(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: bool) -> Result<&mut Self, CassError> {
        self.bind_bool_by_name(col, value)
    }
}

impl BindRustType<f32> for Statement {
    fn bind(&mut self, index: u64, value: f32) -> Result<&mut Self, CassError> {
        self.bind_float(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: f32) -> Result<&mut Self, CassError> {
        self.bind_float_by_name(col, value)
    }
}

impl BindRustType<f64> for Statement {
    fn bind(&mut self, index: u64, value: f64) -> Result<&mut Self, CassError> {
        self.bind_double(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: f64) -> Result<&mut Self, CassError> {
        self.bind_double_by_name(col, value)
    }
}

impl BindRustType<i32> for Statement {
    fn bind(&mut self, index: u64, value: i32) -> Result<&mut Self, CassError> {
        self.bind_int32(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: i32) -> Result<&mut Self, CassError> {
        self.bind_int32_by_name(col, value)
    }
}

impl BindRustType<i64> for Statement {
    fn bind(&mut self, index: u64, value: i64) -> Result<&mut Self, CassError> {
        self.bind_int64(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: i64) -> Result<&mut Self, CassError> {
        self.bind_int64_by_name(col, value)
    }
}

impl<'a> BindRustType<&'a str> for Statement {
    fn bind(&mut self, index: u64, value: &str) -> Result<&mut Self, CassError> {
        self.bind_string(index, &value)
    }

    fn bind_by_name(&mut self, col: &str, value: &str) -> Result<&mut Self, CassError> {
        self.bind_string_by_name(col, &value)
    }
}

impl BindRustType<Set> for Statement {
    fn bind(&mut self, index: u64, value: Set) -> Result<&mut Self, CassError> {
        self.bind_set(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: Set) -> Result<&mut Self, CassError> {
        self.bind_set_by_name(col, value)
    }
}

impl BindRustType<Uuid> for Statement {
    fn bind(&mut self, index: u64, value: Uuid) -> Result<&mut Self, CassError> {
        self.bind_uuid(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: Uuid) -> Result<&mut Self, CassError> {
        self.bind_uuid_by_name(col, value)
    }
}

impl BindRustType<Map> for Statement {
    fn bind(&mut self, index: u64, value: Map) -> Result<&mut Self, CassError> {
        self.bind_map(index, value)
    }

    fn bind_by_name(&mut self, col: &str, value: Map) -> Result<&mut Self, CassError> {
        self.bind_map_by_name(col, value)
    }
}


impl Statement {
    ///Creates a new query statement.
    pub fn new(query: &str, parameter_count: u64) -> Self {
        unsafe {
            Statement(cass_statement_new(CString::new(query).expect("must be utf8").as_ptr(),
                                         parameter_count))
        }
    }

    unsafe fn free(&mut self) {
        cass_statement_free(self.0)
    }

    //    ///Binds an arbitrary CassBindable type to a cassandra statement
    //    ///FIXME not yet implemented
    //    pub fn bind(&mut self, params: Vec<CassBindable>) {
    //        let _ = params;
    //        unimplemented!();
    //    }

    ///Adds a key index specifier to this a statement.
    ///When using token-aware routing, this can be used to tell the driver which
    ///parameters within a non-prepared, parameterized statement are part of
    ///the partition key.
    ///
    ///Use consecutive calls for composite partition keys.
    ///
    ///This is not necessary for prepared statements, as the key
    ///parameters are determined in the metadata processed in the prepare phase.
    pub fn add_key_index(&mut self, index: u64) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_statement_add_key_index(self.0, index)).wrap(self) }
    }

    ///Sets the statement's keyspace for use with token-aware routing.
    ///
    ///This is not necessary for prepared statements, as the keyspace
    ///is determined in the metadata processed in the prepare phase.
    pub fn set_keyspace(&mut self, keyspace: String) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_statement_set_keyspace(self.0,
                                                         (CString::new(keyspace).expect("must be utf8").as_ptr())))
                .wrap(self)
        }
    }

    ///Sets the statement's consistency level.
    ///
    ///<b>Default:</b> CASS_CONSISTENCY_LOCAL_ONE
    pub fn set_consistency(&mut self, consistency: Consistency) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_statement_set_consistency(self.0, consistency.inner())).wrap(self) }
    }

    /// Sets the statement's serial consistency level.
    ///
    ///<b>Default:</b> Not set
    pub fn set_serial_consistency(&mut self, serial_consistency: Consistency) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_set_serial_consistency(self.0, serial_consistency.inner())).wrap(self)
        }
    }

    ///Sets the statement's page size.
    ///
    ///<b>Default:</b> -1 (Disabled)
    pub fn set_paging_size(&mut self, page_size: i32) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_set_paging_size(self.0, page_size)).wrap(self) }
    }

    /// Sets the statement's paging state. This can be used to get the next page of
    ///data in a multi-page query.
    pub fn set_paging_state(&mut self, result: CassResult) -> Result<&mut Self, CassError> {
        unsafe {
            try!(CassError::build(cass_statement_set_paging_state(self.0, result.inner())).wrap(()));
            Ok(self)
        }
    }

    ///Sets the statement's paging state. This can be used to get the next page of
    ///data in a multi-page query.
    ///
    ///<b>Warning:</b> The paging state should not be exposed to or come from
    ///untrusted environments. The paging state could be spoofed and potentially
    ///used to gain access to other data.
    pub fn set_paging_state_token(&mut self, paging_state: &str) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_statement_set_paging_state_token(self.0,
                                                                   paging_state.as_ptr() as *const i8,
                                                                   paging_state.len() as u64))
                .wrap(self)
        }
    }

    ///Sets the statement's timestamp.
    pub fn set_timestamp(&mut self, timestamp: i64) -> Result<&mut Self, CassError> {
        unsafe {
            try!(CassError::build(cass_statement_set_timestamp(self.0, timestamp)).wrap(()));
            Ok(self)
        }
    }

    /// Sets the statement's retry policy.
    pub fn set_retry_policy(&mut self, retry_policy: RetryPolicy) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_statement_set_retry_policy(self.0, retry_policy.inner())).wrap(self) }
    }

    ///Sets the statement's custom payload.
    pub fn set_custom_payload(&mut self, payload: CustomPayload) -> Result<&Self, CassError> {
        unsafe { CassError::build(cass_statement_set_custom_payload(self.0, payload.inner())).wrap(self) }
    }

    ///Binds null to a query or bound statement at the specified index.
    pub fn bind_null(&mut self, index: u64) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_null(self.0, index)).wrap(self) }
    }

    ///Binds a null to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_null_by_name(&mut self, name: &str) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_null_by_name(self.0,
                                                              CString::new(name).expect("must be utf8").as_ptr()))
                .wrap(self)
        }
    }

    ///Binds a "tinyint" to a query or bound statement at the specified index.
    pub fn bind_int8(&mut self, index: u64, value: i8) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_int8(self.0, index, value)).wrap(self) }
    }

    ///Binds a "tinyint" to all the values with the specified name.
    pub fn bind_int8_by_name(&mut self, name: &str, value: i8) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_int8_by_name(self.0,
                                                              CString::new(name).expect("must be utf8").as_ptr(),
                                                              value))
                .wrap(self)
        }
    }

    ///Binds an "smallint" to a query or bound statement at the specified index.
    pub fn bind_int16(&mut self, index: u64, value: i16) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_int16(self.0, index, value)).wrap(self) }
    }

    ///Binds a "smallint" to all the values with the specified name.
    pub fn bind_int16_by_name(&mut self, name: &str, value: i16) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_int16_by_name(self.0,
                                                               CString::new(name).expect("must be utf8").as_ptr(),
                                                               value))
                .wrap(self)
        }
    }

    ///Binds an "int" to a query or bound statement at the specified index.
    pub fn bind_int32(&mut self, index: u64, value: i32) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_int32(self.0, index, value)).wrap(self) }
    }

    ///Binds an "int" to all the values with the specified name.
    pub fn bind_int32_by_name(&mut self, name: &str, value: i32) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_int32_by_name(self.0,
                                                               CString::new(name).expect("must be utf8").as_ptr(),
                                                               value))
                .wrap(self)
        }
    }

    ///Binds a "date" to a query or bound statement at the specified index.
    pub fn bind_uint32(&mut self, index: u64, value: u32) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_uint32(self.0, index, value)).wrap(self) }
    }

    ///Binds a "date" to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_uint32_by_name(&mut self, name: &str, value: u32) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_uint32_by_name(self.0,
                                                                CString::new(name).expect("must be utf8").as_ptr(),
                                                                value))
                .wrap(self)
        }
    }

    ///Binds a "bigint", "counter", "timestamp" or "time" to a query or
    ///bound statement at the specified index.
    pub fn bind_int64(&mut self, index: u64, value: i64) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_int64(self.0, index, value)).wrap(self) }
    }

    ///Binds a "bigint", "counter", "timestamp" or "time" to all values
    ///with the specified name.
    pub fn bind_int64_by_name(&mut self, name: &str, value: i64) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_int64_by_name(self.0,
                                                               CString::new(name).expect("must be utf8").as_ptr(),
                                                               value))
                .wrap(self)
        }
    }

    ///Binds a "float" to a query or bound statement at the specified index.
    pub fn bind_float(&mut self, index: u64, value: f32) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_float(self.0, index, value)).wrap(self) }
    }

    /// Binds a "float" to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_float_by_name(&mut self, name: &str, value: f32) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_float_by_name(self.0,
                                                               CString::new(name).expect("must be utf8").as_ptr(),
                                                               value))
                .wrap(self)
        }
    }

    ///Binds a "double" to a query or bound statement at the specified index.
    pub fn bind_double(&mut self, index: u64, value: f64) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_double(self.0, index, value)).wrap(self) }
    }

    ///Binds a "double" to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_double_by_name(&mut self, name: &str, value: f64) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_double_by_name(self.0,
                                                                CString::new(name).expect("must be utf8").as_ptr(),
                                                                value))
                .wrap(self)
        }
    }

    ///Binds a "boolean" to a query or bound statement at the specified index.
    pub fn bind_bool(&mut self, index: u64, value: bool) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_bool(self.0, index, if value { cass_true } else { cass_false }))
                .wrap(self)
        }
    }

    /// Binds a "boolean" to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    /// cass_prepared_bind().
    pub fn bind_bool_by_name(&mut self, name: &str, value: bool) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_bool_by_name(self.0,
                                                              CString::new(name).expect("must be utf8").as_ptr(),
                                                              if value { cass_true } else { cass_false }))
                .wrap(self)
        }
    }

    ///Binds an "ascii", "text" or "varchar" to a query or bound statement
    ///at the specified index.
    pub fn bind_string(&mut self, index: u64, value: &str) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_string(self.0,
                                                        index,
                                                        CString::new(value).expect("must be utf8").as_ptr()))
                .wrap(self)
        }
    }

    ///Binds an "ascii", "text" or "varchar" to all the values
    ///with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_string_by_name(&mut self, name: &str, value: &str) -> Result<&mut Self, CassError> {
        unsafe {
            let result = cass_statement_bind_string_by_name(self.0,
                                                            CString::new(name).expect("must be utf8").as_ptr(),
                                                            CString::new(value).expect("must be utf8").as_ptr());
            CassError::build(result).wrap(self)
        }
    }

    ///Binds a "blob", "varint" or "custom" to a query or bound statement at the specified index.
    pub fn bind_bytes(&mut self, index: u64, value: Vec<u8>) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_bytes(self.0, index, value.as_ptr(), value.len() as u64)).wrap(self)
        }
    }

    ///Binds a "blob", "varint" or "custom" to all the values with the
    ///specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_bytes_by_name(&mut self, name: &str, mut value: Vec<u8>) -> Result<&mut Self, CassError> {
        unsafe {
            let result = cass_statement_bind_bytes_by_name(self.0,
                                                           CString::new(name).expect("must be utf8").as_ptr(),
                                                           value.as_mut_ptr(),
                                                           value.len() as u64);
            CassError::build(result).wrap(self)
        }
    }

    ///Binds a "uuid" or "timeuuid" to a query or bound statement at the specified index.
    pub fn bind_uuid(&mut self, index: u64, value: Uuid) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_uuid(self.0, index, value.inner())).wrap(self) }
    }

    ///Binds a "uuid" or "timeuuid" to all the values
    ///with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_uuid_by_name(&mut self, name: &str, value: Uuid) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_uuid_by_name(self.0,
                                                              CString::new(name).expect("must be utf8").as_ptr(),
                                                              value.inner()))
                .wrap(self)
        }
    }

    ///Binds an "inet" to a query or bound statement at the specified index.
    pub fn bind_inet(&mut self, index: u64, value: Inet) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_inet(self.0, index, value.inner())).wrap(self) }
    }

    ///Binds an "inet" to all the values with the specified name.
    pub fn bind_inet_by_name(&mut self, name: &str, value: Inet) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_inet_by_name(self.0,
                                                              CString::new(name).expect("must be utf8").as_ptr(),
                                                              value.inner()))
                .wrap(self)
        }
    }


    // 	///Bind a "decimal" to a query or bound statement at the specified index.
    //    pub fn bind_decimal(&self,
    //                                index: i32,
    //                                value: d128)
    //                                -> Result<&mut Self, CassError> {
    //            unsafe {
    //                CassError::build(
    //                    cass_statement_bind_decimal(
    //                        self.0,
    //                        index,
    //                        value
    //                    )
    //                ).wrap(&mut self)
    //            }
    //        }

    // Binds a "decimal" to all the values with the specified name.
    //
    // This can only be used with statements created by
    // cass_prepared_bind().
    //    pub fn bind_decimal_by_name<'a>(&'a self,
    //                                    name: &str,
    //                                    value: String)
    //                                    -> Result<&'a Self, CassError> {
    //        unsafe {
    //            let name = CString::new(name).unwrap();
    //            CassError::build(
    //            cass_statement_bind_decimal_by_name(
    //                self.0,
    //                name.as_ptr(),
    //                value
    //            )
    //        ).wrap(&self)
    //        }
    //    }

    ///Bind a "map" to a query or bound statement at the specified index.
    pub fn bind_map(&mut self, index: u64, map: Map) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_collection(self.0, index, map.inner())).wrap(self) }
    }

    ///Bind a "map" to all the values with the
    ///specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_map_by_name(&mut self, name: &str, map: Map) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_collection_by_name(self.0,
                                                                    CString::new(name).expect("must be utf8").as_ptr(),
                                                                    map.inner()))
                .wrap(self)
        }
    }
    ///Bind a "set" to a query or bound statement at the specified index.
    pub fn bind_set(&mut self, index: u64, collection: Set) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_collection(self.0, index, collection.inner())).wrap(self) }
    }

    ///Bind a "set" to all the values with the
    ///specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_set_by_name(&mut self, name: &str, collection: Set) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_collection_by_name(self.0,
                                                                    CString::new(name).expect("must be utf8").as_ptr(),
                                                                    collection.inner()))
                .wrap(self)
        }
    }

    ///Bind a "list" to a query or bound statement at the specified index.
    pub fn bind_list(&mut self, index: u64, collection: List) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_collection(self.0, index, collection.inner())).wrap(self) }
    }

    ///Bind a "list" to all the values with the
    ///specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_list_by_name(&mut self, name: &str, collection: List) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_collection_by_name(self.0,
                                                                    CString::new(name).expect("must be utf8").as_ptr(),
                                                                    collection.inner()))
                .wrap(self)
        }
    }

    ///Bind a "tuple" to a query or bound statement at the specified index.
    pub fn bind_tuple(&mut self, index: u64, value: Tuple) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_tuple(self.0, index, value.inner())).wrap(self) }
    }

    ///Bind a "tuple" to all the values with the specified name.
    ///
    ///This can only be used with statements created by
    ///cass_prepared_bind().
    pub fn bind_tuple_by_name(&mut self, name: &str, value: Tuple) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_tuple_by_name(self.0,
                                                               CString::new(name).expect("must be utf8").as_ptr(),
                                                               value.inner()))
                .wrap(self)
        }
    }

    ///Bind a user defined type to a query or bound statement at the
    ///specified index.
    pub fn bind_user_type(&mut self, index: u64, value: &UserType) -> Result<&mut Self, CassError> {
        unsafe { CassError::build(cass_statement_bind_user_type(self.0, index, value.inner())).wrap(self) }
    }

    ///Bind a user defined type to a query or bound statement with the
    ///specified name.
    pub fn bind_user_type_by_name(&mut self, name: &str, value: &UserType) -> Result<&mut Self, CassError> {
        unsafe {
            CassError::build(cass_statement_bind_user_type_by_name(self.0,
                                                                   CString::new(name).expect("must be utf8").as_ptr(),
                                                                   value.inner()))
                .wrap(self)
        }
    }
}
