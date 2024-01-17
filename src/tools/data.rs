use super::aws::Aws;
use super::pgpass::PgPass;

#[derive(Debug)]
pub enum Tools {
    PgPass(PgPass),
    Aws(Aws),
}
