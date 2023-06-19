use std::io;

use prost::Message;

use crate::proto::hdfs;

use super::proxy::ProxyEngine;

pub(crate) struct NamenodeProtocol<T: ProxyEngine> {
    engine: T,
}

impl<T: ProxyEngine> NamenodeProtocol<T> {
    pub(crate) fn new(engine: T) -> Self {
        NamenodeProtocol { engine }
    }

    pub(crate) fn get_listing(
        &self,
        src: &str,
        start_after: Vec<u8>,
        need_location: bool,
    ) -> io::Result<hdfs::GetListingResponseProto> {
        let mut message = hdfs::GetListingRequestProto::default();
        message.src = src.to_string();
        message.start_after = start_after;
        message.need_location = need_location;

        let response = self
            .engine
            .call("getListing", message.encode_length_delimited_to_vec())?;
        hdfs::GetListingResponseProto::decode_length_delimited(response)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to decode message"))
    }

    pub(crate) fn get_located_file_info(
        &self,
        src: &str,
    ) -> io::Result<hdfs::GetLocatedFileInfoResponseProto> {
        let mut message = hdfs::GetLocatedFileInfoRequestProto::default();
        message.src = Some(src.to_string());
        message.need_block_token = Some(true);

        let response = self.engine.call(
            "getLocatedFileInfo",
            message.encode_length_delimited_to_vec(),
        )?;
        hdfs::GetLocatedFileInfoResponseProto::decode_length_delimited(response)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Failed to decode message"))
    }
}