use std::ffi::CString;

use cassandra::error::CassError;
use cassandra_sys::CassSsl as _Ssl;
use cassandra_sys::cass_ssl_new;
use cassandra_sys::cass_ssl_free;
use cassandra_sys::cass_ssl_add_trusted_cert;
use cassandra_sys::cass_ssl_set_verify_flags;
use cassandra_sys::cass_ssl_set_cert;
use cassandra_sys::cass_ssl_set_private_key;
use cassandra::util::Protected;

///Describes the SSL configuration of a cluster.
pub struct Ssl(*mut _Ssl);

impl Protected<*mut _Ssl> for Ssl {
    fn inner(&self) -> *mut _Ssl {
        self.0
    }
    fn build(inner: *mut _Ssl) -> Self {
        Ssl(inner)
    }
}


impl Drop for Ssl {
    ///Frees a SSL context instance.
    fn drop(&mut self) {
        unsafe { cass_ssl_free(self.0) }
    }
}

impl Ssl {
    ///Creates a new SSL context.
    pub fn new() -> Ssl {
        unsafe { Ssl(cass_ssl_new()) }
    }

    ///Adds a trusted certificate. This is used to verify
    ///the peer's certificate.
    pub fn add_trusted_cert(&mut self, cert: &str) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_ssl_add_trusted_cert(self.0, CString::new(cert).expect("must be utf8").as_ptr()))
                .wrap(self)
        }
    }

    /// Sets verification performed on the peer's certificate.
    ///
    ///CASS_SSL_VERIFY_NONE - No verification is performed
    ///
    ///CASS_SSL_VERIFY_PEER_CERT - Certificate is present and valid
    ///
    ///CASS_SSL_VERIFY_PEER_IDENTITY - IP address matches the certificate's
    ///common name or one of its subject alternative names. This implies the
    ///certificate is also present.
    ///
    ///<b>Default:</b> CASS_SSL_VERIFY_PEER_CERT
    pub fn set_verify_flags(&mut self, flags: i32) {
        unsafe { cass_ssl_set_verify_flags(self.0, flags) }
    }

    /// Set client-side certificate chain. This is used to authenticate
    /// the client on the server-side. This should contain the entire
    ///Certificate chain starting with the certificate itself.
    pub fn set_cert(&mut self, cert: &str) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_ssl_set_cert(self.0, CString::new(cert).expect("must be utf8").as_ptr())).wrap(self)
        }
    }

    /// Set client-side private key. This is used to authenticate
    ///the client on the server-side.
    pub fn set_private_key(&mut self, key: &str, password: *const i8) -> Result<&Self, CassError> {
        unsafe {
            CassError::build(cass_ssl_set_private_key(self.0,
                                                      CString::new(key).expect("must be utf8").as_ptr(),
                                                      password))
                .wrap(self)
        }
    }
}
