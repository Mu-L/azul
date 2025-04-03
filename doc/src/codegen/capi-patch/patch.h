
/* Macro to turn a compile-time string into a compile-time AzString
 *
 * static AzString foo = AzString_fromConstStr(\"MyString\");
 */
#define AzString_fromConstStr(s) { \
    .vec = { \
        .ptr = s, \
        .len = sizeof(s) - 1, \
        .cap = sizeof(s) - 1, \
        .destructor = AzU8VecDestructor_NoDestructor, \
    } \
}

/* Macro to initialize a compile-time AzNodeData struct
 *
 * static AzNodeData foo = AzNodeData_new(AzNodeType_Div);
 */
#define AzNodeData_new(nt) { \
    .node_type = nt, \
    .dataset = AzOptionRefAny_None, \
    .ids_and_classes = AzIdOrClassVec_empty, \
    .callbacks = AzCallbackDataVec_empty, \
    .inline_css_props = AzNodeDataInlineCssPropertyVec_empty, \
    .clip_mask = AzOptionImageMask_None, \
    .tab_index = AzOptionTabIndex_None, \
}

/* Macro to initialize a compile-time AzDom struct
 *
 * static AzDom foo = AzDom_new(AzNodeType_Div);
 */
#define AzDom_new(nt) { \
    .root = AzNodeData_new(nt),\
    .children = AzDomVec_empty, \
    .total_children = 0, \
}

/* Macro to initialize the default AppConfig struct, must be in a header file
 * so that the LayoutSolverVersion is defined by the binary, not the library -
 * this way upgrading the library won't break the application layout
 *
 * AzAppConfig foo = AzAppConfig_default();
 */
#define AzAppConfig_default(...) { \
    .layout_solver = AzLayoutSolverVersion_March2021, \
    .log_level = AzAppLogLevel_Error, \
    .enable_visual_panic_hook = true, \
    .enable_logging_on_panic = true, \
    .enable_tab_navigation = true, \
    .system_callbacks = AzSystemCallbacks_libraryInternal(), \
}

/* Macro to generate reflection metadata for a given struct - for a "structName" of "foo", generates:
 *
 * constants:
 * - a foo_RttiTypeId, which serves as the "type ID" for that struct
 * - a foo_RttiString, a compile-time string that identifies the class
 *
 * structs:
 * - struct fooRef(): immutable reference to a RefAny<foo>
 * - struct fooRefMut(): mutable reference to a RefAny<foo>
 *
 * functions:
 * - AzRefAny foo_upcast(myStructInstance): upcasts a #structName to a RefAny
 *
 * - fooRef_create(AzRefAny): creates a new fooRef, but does not yet downcast it (.ptr is set to nullptr)
 * - fooRefMut_create(AzRefAny): creates a new fooRefMut, but does not yet downcast it (.ptr is set to nullptr)
 *
 * - bool foo_downcastRef(AzRefAny, fooRef* restrict): downcasts the RefAny immutably, if true is returned then the fooRef is properly initialized
 * - bool foo_downcastMut(AzRefAny, fooRefMut* restrict): downcasts the RefAny mutably, if true is returned then the fooRef is properly initialized
 *
 * - void fooRef_delete(fooRef): disposes of the fooRef and decreases the immutable reference count
 * - void fooRefMut_delete(fooRefMut): disposes of the fooRefMut and decreases the mutable reference count
 * - bool fooRefAny_delete(AzRefAny): disposes of the AzRefAny type, returns false if the AzRefAny is not of type RefAny<foo>
 *
 * USAGE:
 *
 *     typedef struct { } foo;
 *
 *     // -- destructor of foo, azul will call this function once the refcount hits 0
 *     // note: the function expects a void*, but you can just use a foo*
 *     void fooDestructor(foo* restrict destructorPtr) { }
 *
 *     AZ_REFLECT(foo, fooDestructor)
*/
#define AZ_REFLECT(structName, destructor) \
    /* in C all statics are guaranteed to have a unique address, use that address as a TypeId */ \
    static uint64_t const structName##_RttiTypePtrId = 0; \
    static uint64_t const structName##_RttiTypeId = (uint64_t)(&structName##_RttiTypePtrId); \
    static AzString const structName##_Type_RttiString = AzString_fromConstStr(#structName); \
    \
    AzRefAny structName##_upcast(structName const s) { \
        return AzRefAny_newC(&s, sizeof(structName), structName##_RttiTypeId, structName##_Type_RttiString, destructor); \
    } \
    \
    /* generate structNameRef and structNameRefMut structs*/ \
    typedef struct { const structName* ptr; AzRefCount sharing_info; } structName##Ref; \
    typedef struct { structName* restrict ptr; AzRefCount sharing_info; } structName##RefMut; \
    \
    structName##Ref structName##Ref_create(AzRefAny* const refany) { \
        structName##Ref val = { .ptr = 0, .sharing_info = AzRefCount_deepCopy(&refany->sharing_info) };    \
        return val;    \
    } \
    \
    structName##RefMut structName##RefMut_create(AzRefAny* const refany) { \
        structName##RefMut val = { .ptr = 0, .sharing_info = AzRefCount_deepCopy(&refany->sharing_info), };    \
        return val;    \
    } \
    \
    /* if downcastRef returns true, the downcast worked */ \
    bool structName##_downcastRef(AzRefAny* restrict refany, structName##Ref * restrict result) { \
        if (!AzRefAny_isType(refany, structName##_RttiTypeId)) { return false; } else { \
            if (!AzRefCount_canBeShared(&refany->sharing_info)) { return false; } else {\
                AzRefCount_increaseRef(&refany->sharing_info); \
                result->ptr = (structName* const)(refany->_internal_ptr); \
                return true; \
            } \
        } \
    } \
    \
    /* if downcastRefMut returns true, the mutable downcast worked */ \
    bool structName##_downcastMut(AzRefAny* restrict refany, structName##RefMut * restrict result) { \
        if (!AzRefAny_isType(refany, structName##_RttiTypeId)) { return false; } else { \
            if (!AzRefCount_canBeSharedMut(&refany->sharing_info)) { return false; }  else {\
                AzRefCount_increaseRefmut(&refany->sharing_info); \
                result->ptr = (structName* restrict)(refany->_internal_ptr); \
                return true; \
            } \
        } \
    } \
    \
    /* releases a structNameRef (decreases the RefCount) */ \
    void structName##Ref_delete(structName##Ref* restrict value) { \
        AzRefCount_decreaseRef(&value->sharing_info); \
    }\
    \
    /* releases a structNameRefMut (decreases the mutable RefCount) */ \
    void structName##RefMut_delete(structName##RefMut* restrict value) { \
        AzRefCount_decreaseRefmut(&value->sharing_info); \
    }\
    /* releases a structNameRefAny (checks if the RefCount is 0 and calls the destructor) */ \
    bool structName##RefAny_delete(AzRefAny* restrict refany) { \
        if (!AzRefAny_isType(refany, structName##_RttiTypeId)) { return false; } \
        AzRefAny_delete(refany); \
        return true; \
    }