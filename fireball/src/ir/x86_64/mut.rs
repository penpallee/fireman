use crate::{
    ir::{
        x86_64::{X64Mut, X64Range},
        Ir,
    },
    prelude::BitSlice,
};

/// generate_register!(rax);가 들어올 경우 다음을 반환함
/// ```ignore
/// fn rax(&mut self) -> &mut BitSlice {
///     Self::const_bitslice_to_mut(X64::rax(self))
/// }
/// ```
macro_rules! generate_register {
    ($reg:ident) => {
        #[inline(always)]
        fn $reg(&mut self) -> &mut BitSlice {
            unsafe { &mut (*self.register.get())[<Ir as X64Range>::$reg()] }
        }
    };
}

impl X64Mut for Ir {
    generate_register!(rax);
    generate_register!(eax);
    generate_register!(ax);
    generate_register!(al);
    generate_register!(ah);

    generate_register!(rbx);
    generate_register!(ebx);
    generate_register!(bx);
    generate_register!(bl);
    generate_register!(bh);

    generate_register!(rcx);
    generate_register!(ecx);
    generate_register!(cx);
    generate_register!(cl);
    generate_register!(ch);

    generate_register!(rdx);
    generate_register!(edx);
    generate_register!(dx);
    generate_register!(dl);
    generate_register!(dh);

    generate_register!(rsp);
    generate_register!(esp);
    generate_register!(sp);
    generate_register!(spl);

    generate_register!(rbp);
    generate_register!(ebp);
    generate_register!(bp);
    generate_register!(bpl);

    generate_register!(rsi);
    generate_register!(esi);
    generate_register!(si);
    generate_register!(sil);

    generate_register!(rdi);
    generate_register!(edi);
    generate_register!(di);
    generate_register!(dil);

    generate_register!(r8);
    generate_register!(r8d);
    generate_register!(r8w);
    generate_register!(r8b);

    generate_register!(r9);
    generate_register!(r9d);
    generate_register!(r9w);
    generate_register!(r9b);

    generate_register!(r10);
    generate_register!(r10d);
    generate_register!(r10w);
    generate_register!(r10b);

    generate_register!(r11);
    generate_register!(r11d);
    generate_register!(r11w);
    generate_register!(r11b);

    generate_register!(r12);
    generate_register!(r12d);
    generate_register!(r12w);
    generate_register!(r12b);

    generate_register!(r13);
    generate_register!(r13d);
    generate_register!(r13w);
    generate_register!(r13b);

    generate_register!(r14);
    generate_register!(r14d);
    generate_register!(r14w);
    generate_register!(r14b);

    generate_register!(r15);
    generate_register!(r15d);
    generate_register!(r15w);
    generate_register!(r15b);

    generate_register!(cs);
    generate_register!(ds);
    generate_register!(es);
    generate_register!(fs);
    generate_register!(gs);
    generate_register!(ss);

    generate_register!(rip);
    generate_register!(eip);
    generate_register!(ip);

    generate_register!(rflags);
    generate_register!(eflags);
    generate_register!(flags);
    generate_register!(cf);
    generate_register!(pf);
    generate_register!(af);
    generate_register!(zf);
    generate_register!(sf);
    generate_register!(tf);
    generate_register!(r#if);
    generate_register!(df);
    generate_register!(of);
    generate_register!(iopl);
    generate_register!(nt);
    generate_register!(rf);
    generate_register!(vm);
    generate_register!(ac);
    generate_register!(vif);
    generate_register!(vip);
    generate_register!(id);

    generate_register!(less);
    generate_register!(less_or_equal);
    generate_register!(below_or_equal);

    generate_register!(fpu_status_word);
    generate_register!(fpu_ie);
    generate_register!(fpu_de);
    generate_register!(fpu_ze);
    generate_register!(fpu_oe);
    generate_register!(fpu_ue);
    generate_register!(fpu_pe);
    generate_register!(fpu_sf);
    generate_register!(fpu_es);
    generate_register!(fpu_c0);
    generate_register!(fpu_c1);
    generate_register!(fpu_c2);
    generate_register!(fpu_top);
    generate_register!(fpu_c3);
    generate_register!(fpu_b);

    generate_register!(st0);
    generate_register!(st1);
    generate_register!(st2);
    generate_register!(st3);
    generate_register!(st4);
    generate_register!(st5);
    generate_register!(st6);
    generate_register!(st7);

    generate_register!(mm0);
    generate_register!(mm1);
    generate_register!(mm2);
    generate_register!(mm3);
    generate_register!(mm4);
    generate_register!(mm5);
    generate_register!(mm6);
    generate_register!(mm7);

    generate_register!(xmm0);
    generate_register!(xmm1);
    generate_register!(xmm2);
    generate_register!(xmm3);
    generate_register!(xmm4);
    generate_register!(xmm5);
    generate_register!(xmm6);
    generate_register!(xmm7);
    generate_register!(xmm8);
    generate_register!(xmm9);
    generate_register!(xmm10);
    generate_register!(xmm11);
    generate_register!(xmm12);
    generate_register!(xmm13);
    generate_register!(xmm14);
    generate_register!(xmm15);

    generate_register!(cr0);
    generate_register!(cr1);
    generate_register!(cr2);
    generate_register!(cr3);
    generate_register!(cr4);
    generate_register!(cr5);
    generate_register!(cr6);
    generate_register!(cr7);
    generate_register!(cr8);
    generate_register!(cr9);
    generate_register!(cr10);
    generate_register!(cr11);
    generate_register!(cr12);
    generate_register!(cr13);
    generate_register!(cr14);
    generate_register!(cr15);

    generate_register!(dr0);
    generate_register!(dr1);
    generate_register!(dr2);
    generate_register!(dr3);
    generate_register!(dr4);
    generate_register!(dr5);
    generate_register!(dr6);
    generate_register!(dr7);
    generate_register!(dr8);
    generate_register!(dr9);
    generate_register!(dr10);
    generate_register!(dr11);
    generate_register!(dr12);
    generate_register!(dr13);
    generate_register!(dr14);
    generate_register!(dr15);

    generate_register!(tmp8);
    generate_register!(tmp16);
    generate_register!(tmp32);
    generate_register!(tmp64);
}
