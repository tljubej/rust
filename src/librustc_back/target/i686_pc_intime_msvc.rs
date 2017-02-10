// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use target::{Target, TargetResult};

pub fn target() -> TargetResult {
    let mut base = super::windows_msvc_base::opts();
    base.no_default_libraries = true;
    base.cpu = "pentium4".to_string();
    base.max_atomic_width = Some(64);

    base.exe_suffix = ".rta".to_string();
    base.dll_suffix = ".rsl".to_string();

    base.is_builtin = false;

    // Mark all dynamic libraries and executables as compatible with the larger 4GiB address
    // space available to x86 Windows binaries on x86_64.
    base.pre_link_args.push("/LARGEADDRESSAWARE".to_string());
    base.pre_link_args.push("/NODEFAULTLIB".to_string());
    base.pre_link_args.push("/VERSION:21076.20053".to_string());
    base.pre_link_args.push("/SUBSYSTEM:CONSOLE".to_string());

    // Ensure the linker will only produce an image if it can also produce a table of
    // the image's safe exception handlers.
    // https://msdn.microsoft.com/en-us/library/9a89h429.aspx
    base.pre_link_args.push("/SAFESEH".to_string());

    base.pre_link_objects_dll = vec!{"iwin32.lib".to_string(),
                                   "rt.lib".to_string(),
                                   "pcibus.lib".to_string(),
                                   "netlib.lib".to_string(),
                                   "clib.lib".to_string(),
                                   "vshelper.lib".to_string()
                                   };

    base.pre_link_objects_exe = vec!{"iwin32.lib".to_string(),
                                   "rt.lib".to_string(),
                                   "pcibus.lib".to_string(),
                                   "netlib.lib".to_string(),
                                   "clib.lib".to_string(),
                                   "vshelper.lib".to_string()
                                   };


    Ok(Target {
        llvm_target: "i686-pc-intime-msvc".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "32".to_string(),
        data_layout: "e-m:x-p:32:32-i64:64-f80:32-n8:16:32-a:0:32-S32".to_string(),
        arch: "x86".to_string(),
        target_os: "intime".to_string(),
        target_env: "msvc".to_string(),
        target_vendor: "pc".to_string(),
        options: base,
    })
}
