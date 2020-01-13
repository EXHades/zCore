use super::*;
use zircon_object::resource::Resource;

impl Syscall {
    pub fn sys_debuglog_create(
        &self,
        rsrc: usize,
        options: usize,
        target: UserOutPtr<HandleValue>,
    ) -> ZxResult<usize> {
        info!(
            "debuglog_create: resource_handle={:?}, options={:?}",
            rsrc, options,
        );
        let proc = &self.thread.proc;
        let resource = proc.get_object::<Resource>(rsrc as u32)?;
        resource.validate(4)?;
        target.write(1u32)?;
        Ok(ZxError::OK as usize)
    }

    pub fn sys_debuglog_write(
        &self,
        _handle_value: HandleValue,
        _options: u32,
        buf: UserInPtr<u8>,
        len: usize,
    ) -> ZxResult<usize> {
        self.sys_debug_write(buf, len)
    }
}