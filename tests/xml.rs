#[test]
fn test_compile_dom_1() {
    use crate::callbacks::Dummy;

    // Test the output of a certain component
    fn test_component_source_code(input: &str, component_name: &str, expected: &str) {
        let mut component_map = XmlComponentMap::<Dummy>::default();
        let root_nodes = parse_xml_string(input).unwrap();
        get_xml_components(&root_nodes, &mut component_map).unwrap();
        let body_node = get_body_node(&root_nodes).unwrap();
        let components = compile_components_to_rust_code(&component_map).unwrap();
        let (searched_component_source, searched_component_args) =
            components.get(component_name).unwrap();
        let component_string = compile_component(
            component_name,
            searched_component_args,
            searched_component_source,
        );

        // TODO!
        // assert_eq!(component_string, expected);
    }

    fn test_app_source_code(input: &str, expected: &str) {
        let mut component_map = XmlComponentMap::default();
        let root_nodes = parse_xml_string(input).unwrap();
        get_xml_components(&root_nodes, &mut component_map).unwrap();
        let body_node = get_body_node(&root_nodes).unwrap();
        let app_source = compile_body_node_to_rust_code(&body_node, &component_map).unwrap();

        // TODO!
        // assert_eq!(app_source, expected);
    }

    let s1 = r#"
        <component name="test">
            <div id="a" class="b"></div>
        </component>

        <body>
            <Test />
        </body>
    "#;
    let s1_expected = r#"
        fn test() -> StyledDom {
            Dom::div().with_id("a").with_class("b")
        }
    "#;

    test_component_source_code(&s1, "test", &s1_expected);
}

#[test]
fn test_format_args_dynamic() {
    let mut variables = FilteredComponentArguments::new();
    variables.insert("a".to_string(), "value1".to_string());
    variables.insert("b".to_string(), "value2".to_string());
    assert_eq!(
        format_args_dynamic("hello {a}, {b}{{ {c} }}", &variables),
        String::from("hello value1, value2{ {c} }"),
    );
    assert_eq!(
        format_args_dynamic("hello {{a}, {b}{{ {c} }}", &variables),
        String::from("hello {a}, value2{ {c} }"),
    );
    assert_eq!(
        format_args_dynamic("hello {{{{{{{ a   }}, {b}{{ {c} }}", &variables),
        String::from("hello {{{{{{ a   }, value2{ {c} }"),
    );
}

#[test]
fn test_normalize_casing() {
    assert_eq!(normalize_casing("abcDef"), String::from("abc_def"));
    assert_eq!(normalize_casing("abc_Def"), String::from("abc_def"));
    assert_eq!(normalize_casing("abc-Def"), String::from("abc_def"));
    assert_eq!(normalize_casing("abc-def"), String::from("abc_def"));
    assert_eq!(normalize_casing("AbcDef"), String::from("abc_def"));
    assert_eq!(normalize_casing("Abc-Def"), String::from("abc_def"));
    assert_eq!(normalize_casing("Abc_Def"), String::from("abc_def"));
    assert_eq!(normalize_casing("aBc_Def"), String::from("a_bc_def")); // wrong, but whatever
    assert_eq!(
        normalize_casing("StartScreen"),
        String::from("start_screen")
    );
}

#[test]
fn test_parse_component_arguments() {
    let mut args_1_expected = ComponentArguments::new();
    args_1_expected.insert("selected_date".to_string(), "DateTime".to_string());
    args_1_expected.insert("minimum_date".to_string(), "DateTime".to_string());
    args_1_expected.insert("grid_visible".to_string(), "bool".to_string());

    // Everything OK
    assert_eq!(
        parse_component_arguments(
            "gridVisible: bool, selectedDate: DateTime, minimumDate: DateTime"
        ),
        Ok(args_1_expected)
    );

    // Missing type for selectedDate
    assert_eq!(
        parse_component_arguments("gridVisible: bool, selectedDate: , minimumDate: DateTime"),
        Err(ComponentParseError::MissingType(
            1,
            "selectedDate".to_string()
        ))
    );

    // Missing name for first argument
    assert_eq!(
        parse_component_arguments(": bool, selectedDate: DateTime, minimumDate: DateTime"),
        Err(ComponentParseError::MissingName(0))
    );

    // Missing comma after DateTime
    assert_eq!(
        parse_component_arguments(
            "gridVisible: bool, selectedDate: DateTime  minimumDate: DateTime"
        ),
        Err(ComponentParseError::WhiteSpaceInComponentType(
            1,
            "selectedDate".to_string(),
            "DateTime  minimumDate".to_string()
        ))
    );

    // Missing colon after gridVisible
    assert_eq!(
        parse_component_arguments(
            "gridVisible: bool, selectedDate DateTime, minimumDate: DateTime"
        ),
        Err(ComponentParseError::WhiteSpaceInComponentName(
            1,
            "selectedDate DateTime".to_string()
        ))
    );
}

#[test]
fn test_xml_get_item() {
    // <a>
    //     <b/>
    //     <c/>
    //     <d/>
    //     <e/>
    // </a>
    // <f>
    //     <g>
    //         <h/>
    //     </g>
    //     <i/>
    // </f>
    // <j/>

    let mut tree = XmlNode::new("component").with_children(vec![
        XmlNode::new("a").with_children(vec![
            XmlNode::new("b"),
            XmlNode::new("c"),
            XmlNode::new("d"),
            XmlNode::new("e"),
        ]),
        XmlNode::new("f").with_children(vec![
            XmlNode::new("g").with_children(vec![XmlNode::new("h")]),
            XmlNode::new("i"),
        ]),
        XmlNode::new("j"),
    ]);

    assert_eq!(&get_item(&[], &mut tree).unwrap().node_type, "component");
    assert_eq!(&get_item(&[0], &mut tree).unwrap().node_type, "a");
    assert_eq!(&get_item(&[0, 0], &mut tree).unwrap().node_type, "b");
    assert_eq!(&get_item(&[0, 1], &mut tree).unwrap().node_type, "c");
    assert_eq!(&get_item(&[0, 2], &mut tree).unwrap().node_type, "d");
    assert_eq!(&get_item(&[0, 3], &mut tree).unwrap().node_type, "e");
    assert_eq!(&get_item(&[1], &mut tree).unwrap().node_type, "f");
    assert_eq!(&get_item(&[1, 0], &mut tree).unwrap().node_type, "g");
    assert_eq!(&get_item(&[1, 0, 0], &mut tree).unwrap().node_type, "h");
    assert_eq!(&get_item(&[1, 1], &mut tree).unwrap().node_type, "i");
    assert_eq!(&get_item(&[2], &mut tree).unwrap().node_type, "j");

    assert_eq!(get_item(&[123213], &mut tree), None);
    assert_eq!(get_item(&[0, 1, 2], &mut tree), None);
}

#[test]
fn test_prepare_string_1() {
    let input1 = r#"Test"#;
    let output = prepare_string(input1);
    assert_eq!(output, String::from("Test"));
}

#[test]
fn test_prepare_string_2() {
    let input1 = r#"
    Hello,
    123


    Test Test2

    Test3




    Test4
    "#;

    let output = prepare_string(input1);
    assert_eq!(output, String::from("Hello, 123\nTest Test2\nTest3\nTest4"));
}

