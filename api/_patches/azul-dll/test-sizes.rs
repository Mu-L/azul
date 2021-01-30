    impl AzString {
        #[inline]
        pub fn as_str(&self) -> &str {
            unsafe { std::str::from_utf8_unchecked(self.as_bytes()) }
        }
        #[inline]
        pub fn as_bytes(&self) -> &[u8] {
            unsafe { std::slice::from_raw_parts(self.vec.ptr, self.vec.len) }
        }
    }

    impl ::std::fmt::Debug for AzCallback                   { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzLayoutCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzGlCallback                 { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzIFrameCallback             { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzTimerCallback              { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzWriteBackCallback          { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { write!(f, "{:x}", self.cb as usize) }}
    impl ::std::fmt::Debug for AzRefAny                     {
        fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(f, "RefAny {{\r\n")?;
            write!(f, "    _internal_ptr: 0x{:x}\r\n", self._internal_ptr as usize)?;
            write!(f, "    _internal_len: {}\r\n", self._internal_len)?;
            write!(f, "    _internal_layout_size: {}\r\n", self._internal_layout_size)?;
            write!(f, "    _internal_layout_align: {}\r\n", self._internal_layout_align)?;
            write!(f, "    type_name: \"{}\"\r\n", self.type_name.as_str())?;
            write!(f, "    type_id: {}\r\n", self.type_id)?;
            write!(f, "    sharing_info: {:?}\r\n", &self.sharing_info)?;
            write!(f, "    custom_destructor: 0x{:x}\r\n", self.custom_destructor as usize)?;
            write!(f, "}}\r\n")?;
            Ok(())
        }
    }

    impl ::std::fmt::Debug for AzDomVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzDomVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzIdOrClassVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzIdOrClassVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeDataInlineCssPropertyVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeDataInlineCssPropertyVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundContentVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundContentVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundPositionVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundPositionVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundRepeatVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundRepeatVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleBackgroundSizeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleBackgroundSizeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyleTransformVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyleTransformVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssPropertyVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssPropertyVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgMultiPolygonVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgMultiPolygonVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgPathVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgPathVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzVertexAttributeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzVertexAttributeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgPathElementVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgPathElementVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzSvgVertexVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzSvgVertexVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzU32VecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzU32VecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzXWindowTypeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzXWindowTypeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzVirtualKeyCodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzVirtualKeyCodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCascadeInfoVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCascadeInfoVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzScanCodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzScanCodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssDeclarationVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssDeclarationVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssPathSelectorVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssPathSelectorVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStylesheetVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStylesheetVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCssRuleBlockVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCssRuleBlockVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzU8VecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzU8VecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzCallbackDataVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzCallbackDataVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzDebugMessageVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzDebugMessageVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzGLuintVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzGLuintVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzGLintVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzGLintVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStringVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStringVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStringPairVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStringPairVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzLinearColorStopVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzLinearColorStopVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzRadialColorStopVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzRadialColorStopVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeIdVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeIdVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzStyledNodeVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzStyledNodeVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzTagIdsToNodeIdsMappingVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzTagIdsToNodeIdsMappingVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzParentWithNodeDepthVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzParentWithNodeDepthVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
    impl ::std::fmt::Debug for AzNodeDataVecDestructor { fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result { use AzNodeDataVecDestructor::*; match self { DefaultRust => write!(f, "DefaultRust"), NoDestructor => write!(f, "NoDestructor"), External(_) => write!(f, "External"), }}}
