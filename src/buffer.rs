use ffi::*;

use std::mem;
use std::ptr;

pub struct Buffer{
    buffer: *mut GstBuffer
}

impl Drop for Buffer{
    fn drop(&mut self){
        unsafe{
       		gst_mini_object_unref(self.buffer as *mut GstMiniObject);
        }
    }
}

impl Buffer{
    pub unsafe fn new(buffer: *mut GstBuffer, owned: bool) -> Option<Buffer>{
		if buffer != ptr::null_mut(){
	    	if !owned{
        		gst_mini_object_ref(buffer as *mut GstMiniObject);
        	}
	        let buff = Buffer{ buffer: buffer };
	        Some(buff)
	    }else{
	        None
	    }
    }
    
    pub fn map_read<'a,F:FnMut(&::MapInfo)->U,U>(&'a self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, GST_MAP_READ) != 0{
	        	let ret = f(&mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }
    
    pub fn map_write<'a,F:FnMut(&mut ::MapInfo)->U,U>(&'a mut self, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, GST_MAP_WRITE) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }
    
    pub fn map<'a,F:FnMut(&mut ::MapInfo)->U,U>(&'a mut self, flags: ::Map, mut f: F ) -> Result<U,()>{
        unsafe{
	        let mut mapinfo = ::MapInfo::new();
	        if gst_buffer_map(self.buffer, &mut mapinfo, flags as u32) != 0{
	        	let ret = f(&mut mapinfo);
        		gst_buffer_unmap(self.buffer, &mut mapinfo);
        		Ok(ret)
        	}else{
        	    Err(())
        	}
	    }
    }

    pub fn size(&self) -> u64{
        unsafe{ gst_buffer_get_size(self.buffer) }
    }
	
	pub fn len<T>(&self) -> usize{
		(self.size() / mem::size_of::<T>() as u64)  as usize
	}
    
    pub fn gst_buffer(&self) -> *const GstBuffer{
        self.buffer
    }
    
    pub fn gst_buffer_mut(&mut self) -> *mut GstBuffer{
        self.buffer
    }
}

impl ::Transfer<GstBuffer> for Buffer{
    unsafe fn transfer(self) ->  *mut GstBuffer{
        let buffer = self.buffer;
		mem::forget(self);
        buffer
    }
}
