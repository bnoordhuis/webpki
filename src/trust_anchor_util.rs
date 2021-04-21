use crate::{Error, TrustAnchor};

#[deprecated(note = "Use TrustAnchor::try_from_cert_der")]
pub fn cert_der_as_trust_anchor(cert_der: &[u8]) -> Result<TrustAnchor, Error> {
    TrustAnchor::try_from_cert_der(cert_der)
}
