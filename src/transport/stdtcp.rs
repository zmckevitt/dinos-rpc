use crate::transport::{RPCError, RPCHeader, Transport};

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

const RX_BUF_LEN: usize = 8192;
const TX_BUF_LEN: usize = 8192;

impl Transport for TcpStream {
    fn max_send(&self) -> usize {
        RX_BUF_LEN
    }

    fn max_recv(&self) -> usize {
        TX_BUF_LEN
    }

    fn send_msg(&mut self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<(), RPCError> {
        match self.write(&unsafe { hdr.as_bytes() }[..]) {
            Err(_) => return Err(RPCError::TransportError),
            Ok(_) => {}
        };
        for p in payload {
            match self.write(p) {
                Err(_) => return Err(RPCError::TransportError),
                Ok(_) => {}
            };
        }
        Ok(())
    }

    fn try_send_msg(&mut self, hdr: &RPCHeader, payload: &[&[u8]]) -> Result<bool, RPCError> {
        self.send_msg(hdr, payload)?;
        Ok(true)
    }

    fn recv_msg(&mut self, hdr: &mut RPCHeader, payload: &mut [&mut [u8]]) -> Result<(), RPCError> {
        match self.read(unsafe { hdr.as_mut_bytes() }) {
            Err(_) => return Err(RPCError::TransportError),
            Ok(_) => {}
        };
        let expected_bytes = hdr.msg_len as usize;
        Ok(())
    }

    fn try_recv_msg(
        &mut self,
        hdr: &mut RPCHeader,
        payload: &mut [&mut [u8]],
    ) -> Result<bool, RPCError> {
        self.recv_msg(hdr, payload)?;
        Ok(true)
    }

    fn client_connect(&mut self) -> Result<(), RPCError> {
        Ok(())
    }

    fn server_accept(&self) -> Result<(), RPCError> {
        Ok(())
    }
}
