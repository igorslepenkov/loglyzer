
#[test]
fn test_basic_0_0_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"1\\\"\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"-1\":\"bar\",\"1\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_0_1_foo_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"1\\\"[0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"-1\":\"bar\",\"1\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_0_2_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.\\\"-1\\\"\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"-1\":\"bar\",\"1\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_1_0_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_1_1_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"two\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_1_2_three() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"three\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_1_3_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one.two\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_2_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_2_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"one\",\"two\",\"three\"]}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":{\"baz\":\"correct\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.baz\",\"result\":\"correct\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_3_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\\n.\\nbar\\n.baz\",\"result\":\"correct\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_4_foo_bar_baz_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.baz.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_5_foo_bar_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_6_foo_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_7_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_basic_3_8_bad_morebad_morebad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bad.morebad.morebad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"correct\"}}}").unwrap());
    case.assert("basic", data).unwrap();
}


#[test]
fn test_boolean_7_0_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_1_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one <= two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_2_one_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one == one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_3_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one == two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_4_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one > two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_5_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one >= two\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_6_one_two() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one != two\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_7_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two && three > one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_8_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two || three > one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_9_one_two_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"one < two || three < one\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_7_10_two_one_three_one() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"two < one || three < one\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"one\":1,\"three\":3,\"two\":2}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_0_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_1_false_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False && True\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_2_true_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_3_false_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_4_true_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && Number\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_5_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_6_number_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_7_number_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && EmptyList\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_8_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number && True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_9_emptylist_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"EmptyList && True\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_10_emptylist_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"EmptyList && False\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_11_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True || False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_12_true_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True || True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_13_false_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False || True\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_14_false_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"False || False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_15_number_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || EmptyList\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_16_number_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || True\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_17_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || True && False\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_18_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"(Number || True) && False\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_19_number_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Number || (True && False)\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_20_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!True\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_21_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_22_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!Number\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_23_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_24_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && !False\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_25_true_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"True && !EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_26_false_emptylist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!False && !EmptyList\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_27_true_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!(True && False)\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_28_zero() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!Zero\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_8_29_zero() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"!!Zero\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"EmptyList\":[],\"False\":false,\"Number\":5,\"True\":true,\"Zero\":0}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_9_0_outer_empty_string_outer_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.empty_string || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bool\":false,\"empty_list\":[],\"empty_string\":\"\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_9_1_outer_nokey_outer_bool_ou() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.nokey || outer.bool || outer.empty_list || outer.empty_string || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bool\":false,\"empty_list\":[],\"empty_string\":\"\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_0_outer_foo_outer_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo || outer.bar\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_1_outer_foo_outer_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo||outer.bar\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_2_outer_bar_outer_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bar || outer.baz\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_3_outer_bar_outer_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bar||outer.baz\",\"result\":\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_4_outer_bad_outer_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad || outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_5_outer_bad_outer_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad||outer.foo\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_6_outer_foo_outer_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo || outer.bad\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_7_outer_foo_outer_bad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.foo||outer.bad\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_8_outer_bad_outer_alsobad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad || outer.alsobad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_boolean_10_9_outer_bad_outer_alsobad() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"outer.bad||outer.alsobad\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"outer\":{\"bar\":\"bar\",\"baz\":\"baz\",\"foo\":\"foo\"}}").unwrap());
    case.assert("boolean", data).unwrap();
}


#[test]
fn test_current_11_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@\",\"result\":{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_current_11_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@.bar\",\"result\":{\"baz\":\"qux\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_current_11_2_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"@.foo[0]\",\"result\":{\"name\":\"a\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("current", data).unwrap();
}


#[test]
fn test_escape_12_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo.bar\\\"\",\"result\":\"dot\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo bar\\\"\",\"result\":\"space\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_2_foo_nbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\\nbar\\\"\",\"result\":\"newline\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\\\\\"bar\\\"\",\"result\":\"doublequote\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_4_c_windows_path() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"c:\\\\\\\\\\\\\\\\windows\\\\\\\\path\\\"\",\"result\":\"windows\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_5_unix_path() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"/unix/path\\\"\",\"result\":\"unix\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_6_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\\\\\"\\\\\\\"\\\"\",\"result\":\"threequotes\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_escape_12_7_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"bar\\\".\\\"baz\\\"\",\"result\":\"qux\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\\\"\\\"\":\"threequotes\",\"/unix/path\":\"unix\",\"bar\":{\"baz\":\"qux\"},\"c:\\\\\\\\windows\\\\path\":\"windows\",\"foo\\nbar\":\"newline\",\"foo bar\":\"space\",\"foo\\\"bar\":\"doublequote\",\"foo.bar\":\"dot\"}").unwrap());
    case.assert("escape", data).unwrap();
}


#[test]
fn test_filters_13_0_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?@ < `5`]\",\"result\":[0,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_13_1_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?`5` > @]\",\"result\":[0,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_13_2_using_in_a_filter_express() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Using @ in a filter expression\",\"expression\":\"foo[?@ == @]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_14_0_unary_filter_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unary filter expression\",\"expression\":\"foo[?key]\",\"result\":[{\"key\":true},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_14_1_unary_not_filter_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unary not filter expression\",\"expression\":\"foo[?!key]\",\"result\":[{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":null},{\"notkey\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_14_2_equality_with_null_rhs() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Equality with null RHS\",\"expression\":\"foo[?key == `null`]\",\"result\":[{\"key\":null},{\"notkey\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":[]},{\"key\":{}},{\"key\":[0]},{\"key\":{\"a\":\"b\"}},{\"key\":0},{\"key\":1},{\"key\":null},{\"notkey\":true}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_15_0_verify_precedence_of_or_a() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Verify precedence of or/and expressions\",\"expression\":\"foo[?a == `1` || b ==`2` && c == `5`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_15_1_parentheses_can_alter_pre() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Parentheses can alter precedence\",\"expression\":\"foo[?(a == `1` || b ==`2`) && c == `5`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_15_2_not_expressions_combined_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not expressions combined with and/or\",\"expression\":\"foo[?!(a == `1` || b ==`2`)]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_0_filter_with_or_and_and_ex() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with Or and And expressions\",\"expression\":\"foo[?c == `3` || a == `1` && b == `4`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_1_foo_b_2_a_3_b_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?b == `2` || a == `3` && b == `4`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_2_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && b == `4` || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_3_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?(a == `3` && b == `4`) || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_4_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?((a == `3` && b == `4`)) || b == `2`]\",\"result\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_5_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && (b == `4` || b == `2`)]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_16_6_foo_a_3_b_4_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `3` && ((b == `4` || b == `2`))]\",\"result\":[{\"a\":3,\"b\":4}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2,\"c\":3},{\"a\":3,\"b\":4}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_17_0_filter_with_and_expressio() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with and expression\",\"expression\":\"foo[?a == `1` && b == `2`]\",\"result\":[{\"a\":1,\"b\":2}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":3}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_17_1_foo_a_1_b_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a == `1` && b == `4`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":2},{\"a\":1,\"b\":3}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_18_0_filter_with_or_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with or expression\",\"expression\":\"foo[?name == 'a' || name == 'b']\",\"result\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_18_1_foo_name_a_name_e() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?name == 'a' || name == 'e']\",\"result\":[{\"name\":\"a\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_18_2_foo_name_a_name_b_name_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?name == 'a' || name == 'b' || name == 'c']\",\"result\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"},{\"name\":\"c\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_19_0_foo_a_1_b_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a==`1`].b.c\",\"result\":[\"x\",\"y\",\"z\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":1,\"b\":{\"c\":\"x\"}},{\"a\":1,\"b\":{\"c\":\"y\"}},{\"a\":1,\"b\":{\"c\":\"z\"}},{\"a\":2,\"b\":{\"c\":\"z\"}},{\"a\":1,\"baz\":2}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_20_0_foo_bar_1_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`1`].bar[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"baz\":\"other\",\"foo\":[{\"bar\":1},{\"bar\":2},{\"bar\":3},{\"bar\":4},{\"bar\":1,\"baz\":2}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_21_0_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[?bar==`1`]\",\"result\":[[{\"bar\":1,\"foo\":2}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"bar\":2,\"foo\":1},{\"bar\":3,\"foo\":1},{\"bar\":2,\"foo\":1},{\"bar\":1,\"foo\":2}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_21_1_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[*].instances[?bar==`1`]\",\"result\":[[{\"bar\":1,\"foo\":2}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"bar\":2,\"foo\":1},{\"bar\":3,\"foo\":1},{\"bar\":2,\"foo\":1},{\"bar\":1,\"foo\":2}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_21_2_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[?bar==`1`][]\",\"result\":[{\"bar\":1,\"foo\":2}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"bar\":2,\"foo\":1},{\"bar\":3,\"foo\":1},{\"bar\":2,\"foo\":1},{\"bar\":1,\"foo\":2}]}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_0_foo_key_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `true`]\",\"result\":[{\"key\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_1_foo_key_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `false`]\",\"result\":[{\"key\":false}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_2_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `0`]\",\"result\":[{\"key\":0}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_3_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `1`]\",\"result\":[{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_4_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `[0]`]\",\"result\":[{\"key\":[0]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_5_foo_key_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `{\\\"bar\\\": [0]}`]\",\"result\":[{\"key\":{\"bar\":[0]}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_6_foo_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `null`]\",\"result\":[{\"key\":null}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_7_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `[1]`]\",\"result\":[{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_8_foo_key_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key == `{\\\"a\\\":2}`]\",\"result\":[{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_9_foo_true_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`true` == key]\",\"result\":[{\"key\":true}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_10_foo_false_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`false` == key]\",\"result\":[{\"key\":false}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_11_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`0` == key]\",\"result\":[{\"key\":0}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_12_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`1` == key]\",\"result\":[{\"key\":1}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_13_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[0]` == key]\",\"result\":[{\"key\":[0]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_14_foo_bar_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"bar\\\": [0]}` == key]\",\"result\":[{\"key\":{\"bar\":[0]}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_15_foo_null_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`null` == key]\",\"result\":[{\"key\":null}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_16_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[1]` == key]\",\"result\":[{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_17_foo_a_2_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"a\\\":2}` == key]\",\"result\":[{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_18_foo_key_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `true`]\",\"result\":[{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_19_foo_key_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `false`]\",\"result\":[{\"key\":true},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_20_foo_key_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `0`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_21_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `1`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_22_foo_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `null`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_23_foo_key_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `[1]`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_24_foo_key_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?key != `{\\\"a\\\":2}`]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_25_foo_true_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`true` != key]\",\"result\":[{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_26_foo_false_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`false` != key]\",\"result\":[{\"key\":true},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_27_foo_0_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`0` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_28_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`1` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_29_foo_null_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`null` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_30_foo_1_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`[1]` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":{\"a\":2}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_22_31_foo_a_2_key() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?`{\\\"a\\\":2}` != key]\",\"result\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"key\":true},{\"key\":false},{\"key\":0},{\"key\":1},{\"key\":[0]},{\"key\":{\"bar\":[0]}},{\"key\":null},{\"key\":[1]},{\"key\":{\"a\":2}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_23_0_matching_an_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching an expression\",\"expression\":\"foo[?top.first == top.last]\",\"result\":[{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}},{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}},{\"top\":{\"first\":\"foo\",\"last\":\"baz\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_23_1_matching_a_json_array() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a JSON array\",\"expression\":\"foo[?top == `{\\\"first\\\": \\\"foo\\\", \\\"last\\\": \\\"bar\\\"}`]\",\"result\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"first\":\"foo\",\"last\":\"bar\"}},{\"top\":{\"first\":\"foo\",\"last\":\"foo\"}},{\"top\":{\"first\":\"foo\",\"last\":\"baz\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_24_0_filter_with_subexpression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Filter with subexpression\",\"expression\":\"foo[?top.name == 'a']\",\"result\":[{\"top\":{\"name\":\"a\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"top\":{\"name\":\"a\"}},{\"top\":{\"name\":\"b\"}}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_0_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age > `25`]\",\"result\":[{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_1_foo_age_25() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age >= `25`]\",\"result\":[{\"age\":25},{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_2_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age > `30`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_3_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age < `25`]\",\"result\":[{\"age\":20}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_4_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age <= `25`]\",\"result\":[{\"age\":20},{\"age\":25}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_5_greater_than_with_a_numbe() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Greater than with a number\",\"expression\":\"foo[?age < `20`]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_6_foo_age_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age == `20`]\",\"result\":[{\"age\":20}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_25_7_foo_age_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?age != `20`]\",\"result\":[{\"age\":25},{\"age\":30}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"age\":20},{\"age\":25},{\"age\":30}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_26_0_matching_an_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching an expression\",\"expression\":\"foo[?first == last]\",\"result\":[{\"first\":\"foo\",\"last\":\"foo\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"first\":\"foo\",\"last\":\"bar\"},{\"first\":\"foo\",\"last\":\"foo\"},{\"first\":\"foo\",\"last\":\"baz\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_26_1_verify_projection_created() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Verify projection created from filter\",\"expression\":\"foo[?first == last].first\",\"result\":[\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"first\":\"foo\",\"last\":\"bar\"},{\"first\":\"foo\",\"last\":\"foo\"},{\"first\":\"foo\",\"last\":\"baz\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_27_0_matching_a_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a literal\",\"expression\":\"*[?[0] == `0`]\",\"result\":[[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[2,3],\"foo\":[0,1]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_filters_28_0_matching_a_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Matching a literal\",\"expression\":\"foo[?name == 'a']\",\"result\":[{\"name\":\"a\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("filters", data).unwrap();
}


#[test]
fn test_functions_29_0_map_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&[], array)\",\"result\":[[1,2,3,4],[5,6,7,8,9]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[[1,2,3,[4]],[5,6,7,[8,9]]]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_30_0_map_foo_bar_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo.bar, array)\",\"result\":[\"yes1\",\"yes2\",null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_30_1_map_foo1_bar_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo1.bar, array)\",\"result\":[null,null,\"no\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_30_2_map_foo_bar_baz_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo.bar.baz, array)\",\"result\":[null,null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[{\"foo\":{\"bar\":\"yes1\"}},{\"foo\":{\"bar\":\"yes2\"}},{\"foo1\":{\"bar\":\"no\"}}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_31_0_map_a_people() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&a, people)\",\"result\":[10,10,10,10,10,10,10,10,10]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"empty\":[],\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_31_1_map_c_people() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&c, people)\",\"result\":[\"z\",null,null,\"z\",null,null,\"z\",null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"empty\":[],\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_31_2_map_a_badkey() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"map(&a, badkey)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"empty\":[],\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_31_3_map_foo_empty() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"map(&foo, empty)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"empty\":[],\"people\":[{\"a\":10,\"b\":1,\"c\":\"z\"},{\"a\":10,\"b\":2,\"c\":null},{\"a\":10,\"b\":3},{\"a\":10,\"b\":4,\"c\":\"z\"},{\"a\":10,\"b\":5,\"c\":null},{\"a\":10,\"b\":6},{\"a\":10,\"b\":7,\"c\":\"z\"},{\"a\":10,\"b\":8,\"c\":null},{\"a\":10,\"b\":9}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_32_0_stable_sort_order() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"stable sort order\",\"expression\":\"sort_by(people, &age)\",\"result\":[{\"age\":10,\"order\":\"1\"},{\"age\":10,\"order\":\"2\"},{\"age\":10,\"order\":\"3\"},{\"age\":10,\"order\":\"4\"},{\"age\":10,\"order\":\"5\"},{\"age\":10,\"order\":\"6\"},{\"age\":10,\"order\":\"7\"},{\"age\":10,\"order\":\"8\"},{\"age\":10,\"order\":\"9\"},{\"age\":10,\"order\":\"10\"},{\"age\":10,\"order\":\"11\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":10,\"order\":\"1\"},{\"age\":10,\"order\":\"2\"},{\"age\":10,\"order\":\"3\"},{\"age\":10,\"order\":\"4\"},{\"age\":10,\"order\":\"5\"},{\"age\":10,\"order\":\"6\"},{\"age\":10,\"order\":\"7\"},{\"age\":10,\"order\":\"8\"},{\"age\":10,\"order\":\"9\"},{\"age\":10,\"order\":\"10\"},{\"age\":10,\"order\":\"11\"}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_0_sort_by_field_expression() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"sort by field expression\",\"expression\":\"sort_by(people, &age)\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_1_sort_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &age_str)\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_2_sort_by_function_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"sort by function expression\",\"expression\":\"sort_by(people, &to_number(age_str))\",\"result\":[{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3},{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_3_function_projection_on_so() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on sort_by function\",\"expression\":\"sort_by(people, &age)[].name\",\"result\":[3,\"a\",\"c\",\"b\",\"d\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_4_sort_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort_by(people, &extra)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_5_sort_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort_by(people, &bool)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_6_sort_by_people_name() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort_by(people, &name)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_7_sort_by_people_name() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort_by(people, name)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_8_sort_by_people_age_extra() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(people, &age)[].extra\",\"result\":[\"foo\",\"bar\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_9_sort_by_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort_by(`[]`, &age)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_10_max_by_people_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &age)\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_11_max_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &age_str)\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_12_max_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"max_by(people, &bool)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_13_max_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"max_by(people, &extra)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_14_max_by_people_to_number_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max_by(people, &to_number(age_str))\",\"result\":{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_15_min_by_people_age() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &age)\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_16_min_by_people_age_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &age_str)\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_17_min_by_people_bool() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"min_by(people, &bool)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_18_min_by_people_extra() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"min_by(people, &extra)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_33_19_min_by_people_to_number_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min_by(people, &to_number(age_str))\",\"result\":{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"people\":[{\"age\":20,\"age_str\":\"20\",\"bool\":true,\"extra\":\"foo\",\"name\":\"a\"},{\"age\":40,\"age_str\":\"40\",\"bool\":false,\"extra\":\"bar\",\"name\":\"b\"},{\"age\":30,\"age_str\":\"30\",\"bool\":true,\"name\":\"c\"},{\"age\":50,\"age_str\":\"50\",\"bool\":false,\"name\":\"d\"},{\"age\":10,\"age_str\":\"10\",\"bool\":true,\"name\":3}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_34_0_function_projection_on_va() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on variadic function\",\"expression\":\"foo[].not_null(f, e, d, c, b, a)\",\"result\":[\"b\",\"c\",\"d\",\"e\",\"f\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"a\":\"a\",\"b\":\"b\"},{\"b\":\"b\",\"c\":\"c\"},{\"c\":\"c\",\"d\":\"d\"},{\"d\":\"d\",\"e\":\"e\"},{\"e\":\"e\",\"f\":\"f\"}]}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_0_abs_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(foo)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_1_abs_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(foo)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_2_abs_str() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"abs(str)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_3_abs_array_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(array[1])\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_4_abs_array_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(array[1])\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_5_abs_false() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"abs(`false`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_6_abs_24() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`-24`)\",\"result\":24}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_7_abs_24() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"abs(`-24`)\",\"result\":24}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_8_abs_1_2() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-arity\",\"expression\":\"abs(`1`, `2`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_9_abs() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-arity\",\"expression\":\"abs()\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_10_unknown_function_1_2() {
    let case: TestCase = TestCase::from_str("{\"error\":\"unknown-function\",\"expression\":\"unknown_function(`1`, `2`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_11_avg_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"avg(numbers)\",\"result\":2.75}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_12_avg_array() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"avg(array)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_13_avg_abc() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"avg('abc')\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_14_avg_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"avg(foo)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_15_avg() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"avg(@)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_16_avg_strings() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"avg(strings)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_17_ceil_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(`1.2`)\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_18_ceil_decimals_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[0])\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_19_ceil_decimals_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[1])\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_20_ceil_decimals_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ceil(decimals[2])\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_21_ceil_string() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"ceil('string')\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_22_contains_abc_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains('abc', 'a')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_23_contains_abc_d() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains('abc', 'd')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_24_contains_false_d() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"contains(`false`, 'd')\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_25_contains_strings_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(strings, 'a')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_26_contains_decimals_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(decimals, `1.2`)\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_27_contains_decimals_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"contains(decimals, `false`)\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_28_ends_with_str_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'r')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_29_ends_with_str_tr() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'tr')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_30_ends_with_str_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'Str')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_31_ends_with_str_sstr() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'SStr')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_32_ends_with_str_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"ends_with(str, 'foo')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_33_ends_with_str_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"ends_with(str, `0`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_34_floor_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(`1.2`)\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_35_floor_string() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"floor('string')\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_36_floor_decimals_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(decimals[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_37_floor_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"floor(foo)\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_38_floor_str() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"floor(str)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_39_length_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('abc')\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_40_length_okfoo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('✓foo')\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_41_length() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length('')\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_42_length() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(@)\",\"result\":12}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_43_length_strings_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(strings[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_44_length_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(str)\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_45_length_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(array)\",\"result\":6}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_46_length_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(objects)\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_47_length_false() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"length(`false`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_48_length_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"length(foo)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_49_length_strings_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"length(strings[0])\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_50_max_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(numbers)\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_51_max_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(decimals)\",\"result\":1.2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_52_max_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(strings)\",\"result\":\"c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_53_max_abc() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"max(abc)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_54_max_array() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"max(array)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_55_max_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(decimals)\",\"result\":1.2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_56_max_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"max(empty_list)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_57_merge() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{}`)\",\"result\":{}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_58_merge() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{}`, `{}`)\",\"result\":{}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_59_merge_a_1_b_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1}`, `{\\\"b\\\": 2}`)\",\"result\":{\"a\":1,\"b\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_60_merge_a_1_a_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1}`, `{\\\"a\\\": 2}`)\",\"result\":{\"a\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_61_merge_a_1_b_2_a_2_c_3_d_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"merge(`{\\\"a\\\": 1, \\\"b\\\": 2}`, `{\\\"a\\\": 2, \\\"c\\\": 3}`, `{\\\"d\\\": 4}`)\",\"result\":{\"a\":2,\"b\":2,\"c\":3,\"d\":4}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_62_min_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(numbers)\",\"result\":-1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_63_min_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(decimals)\",\"result\":-1.5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_64_min_abc() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"min(abc)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_65_min_array() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"min(array)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_66_min_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(empty_list)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_67_min_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(decimals)\",\"result\":-1.5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_68_min_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"min(strings)\",\"result\":\"a\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_69_type_abc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type('abc')\",\"result\":\"string\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_70_type_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`1.0`)\",\"result\":\"number\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_71_type_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`2`)\",\"result\":\"number\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_72_type_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`true`)\",\"result\":\"boolean\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_73_type_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`false`)\",\"result\":\"boolean\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_74_type_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`null`)\",\"result\":\"null\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_75_type_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`[0]`)\",\"result\":\"array\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_76_type_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(`{\\\"a\\\": \\\"b\\\"}`)\",\"result\":\"object\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_77_type() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"type(@)\",\"result\":\"object\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_78_sort_keys_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(keys(objects))\",\"result\":[\"bar\",\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_79_keys_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"keys(foo)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_80_keys_strings() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"keys(strings)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_81_keys_false() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"keys(`false`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_82_sort_values_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(values(objects))\",\"result\":[\"bar\",\"baz\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_83_keys_empty_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"keys(empty_hash)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_84_values_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"values(foo)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_85_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(', ', strings)\",\"result\":\"a, b, c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_86_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(', ', strings)\",\"result\":\"a, b, c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_87_join_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join(',', `[\\\"a\\\", \\\"b\\\"]`)\",\"result\":\"a,b\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_88_join_a_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"join(',', `[\\\"a\\\", 0]`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_89_join_str() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"join(', ', str)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_90_join_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', strings)\",\"result\":\"a|b|c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_91_join_2_strings() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"join(`2`, strings)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_92_join_decimals() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"join('|', decimals)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_93_join_decimals_to_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', decimals[].to_string(@))\",\"result\":\"1.01|1.2|-1.5\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_94_join_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"join('|', empty_list)\",\"result\":\"\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_95_reverse_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(numbers)\",\"result\":[5,4,3,-1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_96_reverse_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(array)\",\"result\":[\"100\",\"a\",5,4,3,-1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_97_reverse() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse(`[]`)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_98_reverse() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse('')\",\"result\":\"\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_99_reverse_hello_world() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reverse('hello world')\",\"result\":\"dlrow olleh\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_100_starts_with_str_s() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'S')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_101_starts_with_str_st() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'St')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_102_starts_with_str_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'Str')\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_103_starts_with_str_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"starts_with(str, 'String')\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_104_starts_with_str_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"starts_with(str, `0`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_105_sum_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(numbers)\",\"result\":11}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_106_sum_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(decimals)\",\"result\":0.71}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_107_sum_array() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sum(array)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_108_sum_array_to_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(array[].to_number(@))\",\"result\":111}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_109_sum() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sum(`[]`)\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_110_to_array_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array('foo')\",\"result\":[\"foo\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_111_to_array_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(`0`)\",\"result\":[0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_112_to_array_objects() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(objects)\",\"result\":[{\"bar\":\"baz\",\"foo\":\"bar\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_113_to_array_1_2_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(`[1, 2, 3]`)\",\"result\":[1,2,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_114_to_array_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_array(false)\",\"result\":[false]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_115_to_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string('foo')\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_116_to_string_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string(`1.2`)\",\"result\":\"1.2\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_117_to_string_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_string(`[0, 1]`)\",\"result\":\"[0,1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_118_to_number_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('1.0')\",\"result\":1.0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_119_to_number_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('1.1')\",\"result\":1.1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_120_to_number_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('4')\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_121_to_number_notanumber() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number('notanumber')\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_122_to_number_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`false`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_123_to_number_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`null`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_124_to_number_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`[0]`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_125_to_number_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"to_number(`{\\\"foo\\\": 0}`)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_126_to_string_1_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"\\\"to_string\\\"(`1.0`)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_127_sort_numbers() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(numbers)\",\"result\":[-1,3,4,5]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_128_sort_strings() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(strings)\",\"result\":[\"a\",\"b\",\"c\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_129_sort_decimals() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(decimals)\",\"result\":[-1.5,1.01,1.2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_130_sort_array() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort(array)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_131_sort_abc() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort(abc)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_132_sort_empty_list() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sort(empty_list)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_133_sort() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-type\",\"expression\":\"sort(@)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_134_not_null_unknown_key_str() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, str)\",\"result\":\"Str\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_135_not_null_unknown_key_foo_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, foo.bar, empty_list, str)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_136_not_null_unknown_key_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(unknown_key, null_key, empty_list, str)\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_137_not_null_all_expressions_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_null(all, expressions, are_null)\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_138_not_null() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-arity\",\"expression\":\"not_null()\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_139_function_projection_on_si() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on single arg function\",\"expression\":\"numbers[].to_string(@)\",\"result\":[\"-1\",\"3\",\"4\",\"5\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_functions_35_140_function_projection_on_si() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function projection on single arg function\",\"expression\":\"array[].to_number(@)\",\"result\":[-1,3,4,5,100]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[-1,3,4,5,\"a\",\"100\"],\"decimals\":[1.01,1.2,-1.5],\"empty_hash\":{},\"empty_list\":[],\"false\":false,\"foo\":-1,\"null_key\":null,\"numbers\":[-1,3,4,5],\"objects\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"str\":\"Str\",\"strings\":[\"a\",\"b\",\"c\"],\"zero\":0}").unwrap());
    case.assert("functions", data).unwrap();
}


#[test]
fn test_identifiers_36_0_ud834_udd1e() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uD834\\\\uDD1E\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"𝄞\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_37_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_38_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_39_0_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_C\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_C\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_40_0_vh2_h() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"VH2&H\\\\\\\\\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"VH2&H\\\\/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_41_0_su() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sU\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"sU\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_42_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"?\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"?\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_43_0_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"5\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"5\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_44_0_xiuo9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"xIUo9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"xIUo9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_45_0_b7eo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"b7eo\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"b7eo\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_46_0_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\8\\\\\\\\\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\8\\\\\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_47_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"0\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"0\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_48_0_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_49_0_6() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"6\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"6\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_50_0_b_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\\b\\\\n\u{d1a52}\\\\\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\\b\\n\u{d1a52}\\\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_51_0_m_k() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"M_k\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"M_k\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_52_0_9_r_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"9\\\\r\\\\\\\\R\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"9\\r\\\\R\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_53_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"&\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"&\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_54_0_hh() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Hh\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Hh\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_55_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_56_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!,\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!,\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_57_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_F\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_F\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_58_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b%\\\\\\\"\u{9e10f}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b%\\\"\u{9e10f}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_59_0_z9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Z9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Z9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_60_0_tx_uabbb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tX$\\\\uABBb\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tX$ꮻ\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_61_0_bq() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"BQ\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"BQ\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_62_0_w_a0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"W_a0_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"W_a0_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_63_0_i() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"I_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"I_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_64_0_n_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\n\\\\\\\\\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\n\\\\\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_65_0_tk_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tK\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tK\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_66_0_62l() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_62L\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_62L\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_67_0_d7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"D7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"D7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_68_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_69_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b+\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b+\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_70_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_0\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_0\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_71_0_yu_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"YU_2\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"YU_2\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_72_0_z_m() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"z_M_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"z_M_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_73_0_z_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Z_5\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Z_5\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_74_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{f5141}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{f5141}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_75_0_434() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__434\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__434\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_76_0_zs1dc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"zs1DC\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"zs1DC\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_77_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{103c02}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{103c02}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_78_0_bw_6hg_gl() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_BW_6Hg_Gl\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_BW_6Hg_Gl\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_79_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_80_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\":\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\":\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_81_0_uefac() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\u{c77c7}\\\\\\\\ueFAc\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{c77c7}\\\\ueFAc\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_82_0_t_n_b_z() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\/+\\\\t\\\\n\\\\b!Z\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/+\\t\\n\\b!Z\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_83_0_b_q() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"B_q\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"B_q\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_84_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\",\\\\t;\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\",\\t;\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_85_0_bp() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Bp\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Bp\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_86_0_ns_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\nS \\\\n\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\nS \\n\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_87_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"B__\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"B__\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_88_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"#\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"#\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_89_0_t_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t&\\\\\\\\\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t&\\\\\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_90_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_91_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_92_0_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_93_0_gy() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Gy\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Gy\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_94_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_95_0_fa0_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"fa0_9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"fa0_9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_96_0_u_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"U)\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"U)\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_97_0_ueebf() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uEEbF\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\u{eebf}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_98_0_i_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!I\\\\n\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!I\\n/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_99_0_hu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hU\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hU\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_100_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"; !\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"; !\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_101_0_hvu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hvu\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hvu\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_102_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\">\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\">\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_103_0_b_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\\b\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\\b\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_104_0_kl() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Kl\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Kl\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_105_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_106_0_s() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\u{de8a4}S\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\u{de8a4}S\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_107_0_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"7\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"7\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_108_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\"!\\\\/\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\"!/\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_109_0_mg() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Mg\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Mg\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_110_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"+\\\\\\\"\\\\\\\"\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"+\\\"\\\"\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_111_0_r_fb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\\fB \\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\\fB \":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_112_0_m() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"m_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"m_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_113_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_114_0_u4fdc() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\\u4FDc\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\俜\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_115_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\f\u{e5333}\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\f\u{e5333}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_116_0_n() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\n\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\n\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_117_0_obf() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Obf\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Obf\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_118_0_rb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\rB\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\rB\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_119_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\":\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\":\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_120_0_j() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_j\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_j\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_121_0_r_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_r_8\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_r_8\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_122_0_o() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"O_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"O_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_123_0_b_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\b\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\b\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_124_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_125_0_p9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"p9\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"p9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_126_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"-\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"-\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_127_0_r7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"r7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"r7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_128_0_r_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\r\\\\f:\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\r\\f:\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_129_0_rr9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"RR9_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"RR9_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_130_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_131_0_q_7gl8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_Q__7GL8\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_Q__7GL8\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_132_0_q() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Q\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Q\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_133_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_134_0_b_ud8cb_udc83() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\\b\\\\ud8cb\\\\udc83\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\\\\\b\u{42c83}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_135_0_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"9\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"9\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_136_0_sna() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"sNA_\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"sNA_\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_137_0_ubbce_ufafb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\ubBcE\\\\ufAfB\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"믎\u{fafb}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_138_0_u_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"<<U\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"<<U\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_139_0_tl7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"tL7\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"tL7\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_140_0_uaba1_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\uaBA1\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"ꮡ\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_141_0_6w() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_6W\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_6W\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_142_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"R!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"R!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_143_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" [\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" [\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_144_0_tm() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"tM\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"tM\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_145_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_146_0_e4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"E4\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"E4\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_147_0_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\f\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\f\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_148_0_h() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"H\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"H\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_149_0_v24_w() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"v24_W\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"v24_W\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_150_0_t4_ud9da_udd15() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t4\\\\ud9da\\\\udd15\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t4\u{86915}\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_151_0_x() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"_X\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"_X\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_152_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_153_0_v2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"v2\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"v2\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_154_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" \\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" \":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_155_0_t() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\" \\\\t\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\" \\t\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_156_0_tf_ucebb() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\tF\\\\uCebb\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"\\tF캻\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_157_0_x() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"x\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"x\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_158_0_y_1623() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"Y_1623\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"Y_1623\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_159_0_r() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"!\\\\r\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"!\\r\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_identifiers_160_0_l() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"__L\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"__L\":true}").unwrap());
    case.assert("identifiers", data).unwrap();
}


#[test]
fn test_indices_161_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_3_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_4_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_5_hash_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_6_number_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_7_nullvalue_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_161_8_nullvalue_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[].foo[].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_162_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_162_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_162_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar\",\"result\":[[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}],[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_162_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[]\",\"result\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4},{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_162_4_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].baz\",\"result\":[1,3,5,7]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\",\"six\"],[\"seven\",\"eight\"],[\"nine\"],[\"ten\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_1_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0]\",\"result\":[\"one\",\"three\",\"five\",\"seven\",\"nine\",\"ten\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_2_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][1]\",\"result\":[\"two\",\"four\",\"six\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_3_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0][0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_4_foo_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][2][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_163_5_foo_0_0_100() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[][0][0][100]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_0_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].bar\",\"result\":[1,2,4,5,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_1_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].baz\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_2_reservations_instances_no() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].notfoo[].bar\",\"result\":[20,21,22,23,24,25]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_3_reservations_instances_no() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].notfoo[].notbar\",\"result\":[[7],[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_4_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_5_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo[].notbar\",\"result\":[3,[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_6_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].bar[].baz\",\"result\":[[1],[2],[3],[4]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_7_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].baz[].baz\",\"result\":[[1,2],[],[],[3,4]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_8_reservations_instances_qu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].qux[].baz\",\"result\":[[],[1,2,3],[4],[],[],[1,2,3],[4],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_164_9_reservations_instances_qu() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].qux[].baz[]\",\"result\":[1,2,3,4,1,2,3,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"foo\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"foo\":\"bar\"},{\"notfoo\":[{\"bar\":20},{\"bar\":21},{\"notbar\":[7]},{\"bar\":22}]},{\"bar\":[{\"baz\":[1]},{\"baz\":[2]},{\"baz\":[3]},{\"baz\":[4]}]},{\"baz\":[{\"baz\":[1,2]},{\"baz\":[]},{\"baz\":[]},{\"baz\":[3,4]}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}},{\"instances\":[{\"a\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]},{\"b\":[{\"bar\":5},{\"bar\":6},{\"notbar\":[7]},{\"bar\":8}]},{\"c\":\"bar\"},{\"notfoo\":[{\"bar\":23},{\"bar\":24},{\"notbar\":[7]},{\"bar\":25}]},{\"qux\":[{\"baz\":[]},{\"baz\":[1,2,3]},{\"baz\":[4]},{\"baz\":[]}]}],\"otherkey\":{\"foo\":[{\"bar\":1},{\"bar\":2},{\"notbar\":3},{\"bar\":4}]}}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_165_0_reservations_instances_fo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].foo\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_165_1_reservations_instances_ba() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].bar\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_165_2_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_165_3_reservations_notinstances() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].notinstances[].foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"foo\":1},{\"foo\":2}]}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[1]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[2]\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_3_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-1]\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_4_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-2]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_166_5_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[-3]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("[\"one\",\"two\",\"three\"]").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_1_foo_0_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0].bar\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_2_foo_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1].bar\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_3_foo_2_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[2].bar\",\"result\":\"three\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_4_foo_3_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3].notbar\",\"result\":\"four\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_5_foo_3_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_6_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0]\",\"result\":{\"bar\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_7_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1]\",\"result\":{\"bar\":\"two\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_8_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[2]\",\"result\":{\"bar\":\"three\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_9_foo_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[3]\",\"result\":{\"notbar\":\"four\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_167_10_foo_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[4]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0]\",\"result\":\"zero\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_1_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[1]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_2_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[2]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_3_foo_bar_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[3]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_4_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-1]\",\"result\":\"two\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_5_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-2]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_6_foo_bar_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-3]\",\"result\":\"zero\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_indices_168_7_foo_bar_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[-4]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[\"zero\",\"one\",\"two\"]}}").unwrap());
    case.assert("indices", data).unwrap();
}


#[test]
fn test_literal_169_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'foo'\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'  foo  '\",\"result\":\"  foo  \"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_2_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'0'\",\"result\":\"0\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_3_newline() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'newline\\n'\",\"result\":\"newline\\n\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_4_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'\\n'\",\"result\":\"\\n\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_5_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'✓'\",\"result\":\"✓\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_6_g() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'𝄞'\",\"result\":\"𝄞\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_7_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'  [foo]  '\",\"result\":\"  [foo]  \"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_8_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"'[foo]'\",\"result\":\"[foo]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_9_do_not_interpret_escaped_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Do not interpret escaped unicode.\",\"expression\":\"'\\\\u03a6'\",\"result\":\"\\\\u03a6\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_10_can_escape_the_single_quo() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Can escape the single quote\",\"expression\":\"'foo\\\\'bar'\",\"result\":\"foo'bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_11_backslash_not_followed_by() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Backslash not followed by single quote is treated as any other character\",\"expression\":\"'\\\\z'\",\"result\":\"\\\\z\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_169_12_backslash_not_followed_by() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Backslash not followed by single quote is treated as any other character\",\"expression\":\"'\\\\\\\\'\",\"result\":\"\\\\\\\\\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_170_0_literal_with_leading_whit() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal with leading whitespace\",\"expression\":\"`  {\\\"foo\\\": true}`\",\"result\":{\"foo\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_170_1_literal_with_trailing_whi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal with trailing whitespace\",\"expression\":\"`{\\\"foo\\\": true}   `\",\"result\":{\"foo\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_170_2_literal_on_rhs_of_subexpr() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal on RHS of subexpr not allowed\",\"error\":\"syntax\",\"expression\":\"foo.`\\\"bar\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"foo\\\"`\",\"result\":\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_1_interpret_escaped_unicode() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Interpret escaped unicode.\",\"expression\":\"`\\\"\\\\u03a6\\\"`\",\"result\":\"Φ\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_2_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"✓\\\"`\",\"result\":\"✓\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_3_1_2_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`[1, 2, 3]`\",\"result\":[1,2,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_4_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`{\\\"a\\\": \\\"b\\\"}`\",\"result\":{\"a\":\"b\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_5_true() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`true`\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_6_false() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`false`\",\"result\":false}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_7_null() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`null`\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_8_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`0`\",\"result\":0}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_9_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`1`\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_10_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`2`\",\"result\":2}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_11_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`3`\",\"result\":3}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_12_4() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`4`\",\"result\":4}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_13_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`5`\",\"result\":5}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_14_6() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`6`\",\"result\":6}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_15_7() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`7`\",\"result\":7}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_16_8() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`8`\",\"result\":8}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_17_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`9`\",\"result\":9}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_18_escaping_a_backtick_in_qu() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Escaping a backtick in quotes\",\"expression\":\"`\\\"foo\\\\`bar\\\"`\",\"result\":\"foo`bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_19_double_quote_in_literal() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Double quote in literal\",\"expression\":\"`\\\"foo\\\\\\\"bar\\\"`\",\"result\":\"foo\\\"bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_20_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"`\\\"1\\\\`\\\"`\",\"result\":\"1`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_21_multiple_literal_expressi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multiple literal expressions with escapes\",\"expression\":\"`\\\"\\\\\\\\\\\"`.{a:`\\\"b\\\"`}\",\"result\":{\"a\":\"b\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_22_literal_identifier() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier\",\"expression\":\"`{\\\"a\\\": \\\"b\\\"}`.a\",\"result\":\"b\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_23_literal_identifier_identi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier . identifier\",\"expression\":\"`{\\\"a\\\": {\\\"b\\\": \\\"c\\\"}}`.a.b\",\"result\":\"c\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_literal_171_24_literal_identifier_bracke() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"literal . identifier bracket-expr\",\"expression\":\"`[0, 1, 2]`[1]\",\"result\":1}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":\"qux\"},\"foo\":[{\"name\":\"a\"},{\"name\":\"b\"}]}").unwrap());
    case.assert("literal", data).unwrap();
}


#[test]
fn test_multiselect_172_0_nested_multiselect() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multiselect\",\"expression\":\"[[*]]\",\"result\":[[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_172_1_select_on_null() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Select on null\",\"expression\":\"missing.{foo: bar}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_173_0_nested_multiselect() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multiselect\",\"expression\":\"[[*],*]\",\"result\":[null,[\"object\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_174_0_foo_baz_not_there_baz_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].not_there || baz[*].bar, qux[0]]\",\"result\":[[\"a\",\"d\"],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bam\":\"b\",\"bar\":\"a\",\"boo\":\"c\"},{\"bam\":\"e\",\"bar\":\"d\",\"boo\":\"f\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_175_0_foo_baz_bar_boo_qux_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].[bar, boo], qux[0]]\",\"result\":[[[\"a\",\"c\"],[\"d\",\"f\"]],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bam\":\"b\",\"bar\":\"a\",\"boo\":\"c\"},{\"bam\":\"e\",\"bar\":\"d\",\"boo\":\"f\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_176_0_foo_baz_bar_qux_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[baz[*].bar, qux[0]]\",\"result\":[[\"abc\",\"def\"],\"zero\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"baz\":[{\"bar\":\"abc\"},{\"bar\":\"def\"}],\"qux\":[\"zero\"]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[]\",\"result\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar\",\"result\":[[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}],[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[]\",\"result\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4},{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_4_foo_bar_baz_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz, qux]\",\"result\":[[1,2],[3,4],[5,6],[7,8]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_5_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz]\",\"result\":[[1],[3],[5],[7]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_177_6_foo_bar_baz_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].bar[].[baz, qux][]\",\"result\":[1,2,3,4,5,6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":1,\"qux\":2},{\"baz\":3,\"qux\":4}]},{\"bar\":[{\"baz\":5,\"qux\":6},{\"baz\":7,\"qux\":8}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_178_0_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[*].instances[*].{id: id, name: name}\",\"result\":[[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}],[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_178_1_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].{id: id, name: name}\",\"result\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"},{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_178_2_reservations_instances_id() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"reservations[].instances[].[id, name]\",\"result\":[[\"id1\",\"first\"],[\"id2\",\"second\"],[\"id3\",\"third\"],[\"id4\",\"fourth\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"reservations\":[{\"instances\":[{\"id\":\"id1\",\"name\":\"first\"},{\"id\":\"id2\",\"name\":\"second\"}]},{\"instances\":[{\"id\":\"id3\",\"name\":\"third\"},{\"id\":\"id4\",\"name\":\"fourth\"}]}]}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_179_0_foo_bar_bar_baz_1_include() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar.baz[1],includeme: includeme}\",\"result\":{\"bar\":{\"common\":\"second\",\"two\":2},\"includeme\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_179_1_foo_bar_baz_two_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar.baz.two\\\": bar.baz[1].two, includeme: includeme}\",\"result\":{\"bar.baz.two\":2,\"includeme\":true}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_179_2_foo_includeme_bar_baz_com() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[*].common]\",\"result\":[true,[\"first\",\"second\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_179_3_foo_includeme_bar_baz_non() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[*].none]\",\"result\":[true,[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_179_4_foo_includeme_bar_baz_com() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[includeme, bar.baz[].common]\",\"result\":[true,[\"first\",\"second\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":[{\"common\":\"first\",\"one\":1},{\"common\":\"second\",\"two\":2}]},\"ignoreme\":1,\"includeme\":true}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_180_0_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, baz: baz}\",\"result\":{\"bar\":1,\"baz\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":2}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_180_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz]\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":2}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_0_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar:bar,baz:baz}\",\"result\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_1_foo_bar_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[0]]\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_2_foo_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[1]]\",\"result\":[1,3]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_3_foo_bar_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[2]]\",\"result\":[1,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_4_foo_bar_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz[3]]\",\"result\":[1,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_181_5_foo_bar_0_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar[0],baz[3]]\",\"result\":[null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":1,\"baz\":[2,3,4]}}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_0_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar}\",\"result\":{\"bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_1_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar\\\": bar}\",\"result\":{\"bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_2_foo_foo_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"foo.bar\\\": bar}\",\"result\":{\"foo.bar\":\"bar\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_3_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, baz: baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_4_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{\\\"bar\\\": bar, \\\"baz\\\": baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_5_baz_baz_qux_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"baz\\\": baz, \\\"qux\\\\\\\"\\\": \\\"qux\\\\\\\"\\\"}\",\"result\":{\"baz\":2,\"qux\\\"\":3}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_6_foo_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar:bar,baz:baz}\",\"result\":{\"bar\":\"bar\",\"baz\":\"baz\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_7_foo_bar_bar_qux_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar,qux: qux}\",\"result\":{\"bar\":\"bar\",\"qux\":\"qux\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_8_foo_bar_bar_noexist_noexi() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{bar: bar, noexist: noexist}\",\"result\":{\"bar\":\"bar\",\"noexist\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_9_foo_noexist_noexist_alson() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{noexist: noexist, alsonoexist: alsonoexist}\",\"result\":{\"alsonoexist\":null,\"noexist\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_10_foo_badkey_nokey_nokey_al() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.badkey.{nokey: nokey, alsonokey: alsonokey}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_11_foo_nested_a_a_b_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.*.{a: a,b: b}\",\"result\":[{\"a\":\"first\",\"b\":\"second\"},{\"a\":\"first\",\"b\":\"second\"},{\"a\":\"first\",\"b\":\"second\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_12_foo_nested_three_a_a_cinn() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.three.{a: a, cinner: c.inner}\",\"result\":{\"a\":\"first\",\"cinner\":\"third\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_13_foo_nested_three_a_a_c_c_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.nested.three.{a: a, c: c.inner.bad.key}\",\"result\":{\"a\":\"first\",\"c\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_14_foo_a_nested_one_a_b_nest() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.{a: nested.one.a, b: nested.two.b}\",\"result\":{\"a\":\"first\",\"b\":\"second\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_15_bar_bar_baz_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{bar: bar, baz: baz}\",\"result\":{\"bar\":1,\"baz\":2}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_16_bar_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{bar: bar}\",\"result\":{\"bar\":1}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_17_otherkey_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{otherkey: bar}\",\"result\":{\"otherkey\":1}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_18_no_no_exist_exist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{no: no, exist: exist}\",\"result\":{\"exist\":null,\"no\":null}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_19_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar]\",\"result\":[\"bar\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_20_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,baz]\",\"result\":[\"bar\",\"baz\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_21_foo_bar_qux() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,qux]\",\"result\":[\"bar\",\"qux\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_22_foo_bar_noexist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[bar,noexist]\",\"result\":[\"bar\",null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_multiselect_182_23_foo_noexist_alsonoexist() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[noexist,alsonoexist]\",\"result\":[null,null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":1,\"baz\":2,\"foo\":{\"bar\":\"bar\",\"baz\":\"baz\",\"nested\":{\"one\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"},\"three\":{\"a\":\"first\",\"b\":\"second\",\"c\":{\"inner\":\"third\"}},\"two\":{\"a\":\"first\",\"b\":\"second\",\"c\":\"third\"}},\"qux\":\"qux\"},\"qux\\\"\":3}").unwrap());
    case.assert("multiselect", data).unwrap();
}


#[test]
fn test_pipe_183_0_foo_bar_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[*] | [0][0]\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"baz\":\"one\"},{\"baz\":\"two\"}]},{\"bar\":[{\"baz\":\"three\"},{\"baz\":\"four\"}]}]}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | bar\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | bar | baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo|bar| baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_3_not_there_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_there | [0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_4_not_there_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"not_there | [0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_5_foo_bar_foo_other_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[foo.bar, foo.other] | [0]\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_6_a_foo_bar_b_foo_other_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | a\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_7_a_foo_bar_b_foo_other_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | b\",\"result\":{\"baz\":\"two\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_8_foo_bam_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bam || foo.bar | baz\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_184_9_foo_not_there_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo | not_there || bar\",\"result\":{\"baz\":\"one\"}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"one\"},\"other\":{\"baz\":\"two\"},\"other2\":{\"baz\":\"three\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"d\",\"e\",\"f\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_0_foo_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [0]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_1_foo_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [1]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_2_foo_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz | [2]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_3_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.* | [0]\",\"result\":\"subkey\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_4_foo_notbaz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz | [*]\",\"result\":[[\"a\",\"b\",\"c\"],[\"a\",\"b\",\"c\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_pipe_185_5_a_foo_bar_b_foo_other_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"{\\\"a\\\": foo.bar, \\\"b\\\": foo.other} | *.baz\",\"result\":[\"subkey\",\"subkey\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"subkey\"},\"other\":{\"baz\":\"subkey\"},\"other2\":{\"baz\":\"subkey\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]}}}").unwrap());
    case.assert("pipe", data).unwrap();
}


#[test]
fn test_slice_186_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:]\",\"result\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_186_1_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:2].a\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_186_2_1_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[::-1].a\",\"result\":[3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_186_3_2_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[:2].b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"a\":1},{\"a\":2},{\"a\":3}]").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_0_foo_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].a\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_1_foo_2_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_2_foo_2_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:2].a.b\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_3_bar_1_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[::-1].a.b\",\"result\":[3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_4_bar_2_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[:2].a.b\",\"result\":[1,2]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_187_5_baz_2_a() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"baz[:2].a\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":[{\"a\":{\"b\":1}},{\"a\":{\"b\":2}},{\"a\":{\"b\":3}}],\"baz\":50,\"foo\":[{\"a\":1},{\"a\":2},{\"a\":3}]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_0_bar_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[0:10]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_1_foo_0_10_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_2_foo_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_3_foo_0_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_4_foo_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0::1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_5_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0::]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_6_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_7_foo_10_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:10:1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_8_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::1]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_9_foo_10() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:10:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_10_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_11_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_12_foo_1_9() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1:9]\",\"result\":[1,2,3,4,5,6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_13_foo_0_10_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:10:2]\",\"result\":[0,2,4,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_14_foo_5() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[5:]\",\"result\":[5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_15_foo_5_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[5::2]\",\"result\":[5,7,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_16_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::2]\",\"result\":[0,2,4,6,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_17_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[::-1]\",\"result\":[9,8,7,6,5,4,3,2,1,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_18_foo_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[1::2]\",\"result\":[1,3,5,7,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_19_foo_10_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:0:-1]\",\"result\":[9,8,7,6,5,4,3,2,1]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_20_foo_10_5_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:5:-1]\",\"result\":[9,8,7,6]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_21_foo_8_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[8:2:-2]\",\"result\":[8,6,4]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_22_foo_0_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0:20]\",\"result\":[0,1,2,3,4,5,6,7,8,9]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_23_foo_10_20_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:-20:-1]\",\"result\":[9,8,7,6,5,4,3,2,1,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_24_foo_10_20() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[10:-20]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_25_foo_4_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[-4:-1]\",\"result\":[6,7,8]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_26_foo_5_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[:-5:-1]\",\"result\":[9,8,7,6]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_27_foo_8_2_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"invalid-value\",\"expression\":\"foo[8:2:0]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_28_foo_8_2_0_1() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[8:2:0:1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_29_foo_8_2() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[8:2&]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_slice_188_30_foo_2_a_3() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[2:a:3]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"bar\":{\"baz\":1},\"foo\":[0,1,2,3,4,5,6,7,8,9]}").unwrap());
    case.assert("slice", data).unwrap();
}


#[test]
fn test_syntax_189_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*||*|*|*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_189_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[]||[*]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_189_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*.*]\",\"result\":[null]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[]").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_190_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_190_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"foo\\\"\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_190_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"\\\\\\\\\\\"\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_190_3_u() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"\\\"\\\\u\\\"\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_0_bar_anything() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"bar.`\\\"anything\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_1_bar_baz_noexists_literal() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"bar.baz.noexists.`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_2_literal_wildcard_projecti() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal wildcard projection\",\"error\":\"syntax\",\"expression\":\"foo[*].`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_3_foo_name_literal() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[*].name.`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_4_foo_name_literal() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[].name.`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_5_foo_name_literal_subliter() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[].name.`\\\"literal\\\"`.`\\\"subliteral\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_6_projecting_a_literal_onto() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Projecting a literal onto an empty list\",\"error\":\"syntax\",\"expression\":\"foo[*].name.noexist.`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_7_foo_name_noexist_literal() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[].name.noexist.`\\\"literal\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_8_twolen_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"twolen[*].`\\\"foo\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_9_two_level_projection_of_a() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Two level projection of a literal\",\"error\":\"syntax\",\"expression\":\"twolen[*].threelen[*].`\\\"bar\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_10_two_level_flattened_proje() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Two level flattened projection of a literal\",\"error\":\"syntax\",\"expression\":\"twolen[].threelen[].`\\\"bar\\\"`\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_191_11_expects_closing() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"expects closing ]\",\"error\":\"syntax\",\"expression\":\"foo[? @ | @\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_0_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`\\\"baz\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_1_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[? bar == `\\\"baz\\\"` ]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_2_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[ ?bar==`\\\"baz\\\"`]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_3_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[?bar==]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_4_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[?==]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_5_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[?==bar]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_6_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[?bar==baz?]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_7_foo_a_b_c_d_e_f() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?a.b.c==d.e.f]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_8_foo_bar_0_1_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`[0, 1, 2]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_9_foo_bar_a_b_c() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[?bar==`[\\\"a\\\", \\\"b\\\", \\\"c\\\"]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_10_literal_char_not_escaped() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal char not escaped\",\"error\":\"syntax\",\"expression\":\"foo[?bar==`[\\\"foo`bar\\\"]`]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_11_literal_char_escaped() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Literal char escaped\",\"expression\":\"foo[?bar==`[\\\"foo\\\\`bar\\\"]`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_12_unknown_comparator() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unknown comparator\",\"error\":\"syntax\",\"expression\":\"foo[?bar<>baz]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_13_unknown_comparator() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Unknown comparator\",\"error\":\"syntax\",\"expression\":\"foo[?bar^baz]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_14_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[bar==baz]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_15_quoted_identifier_in_filt() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Quoted identifier in filter expression no spaces\",\"expression\":\"[?\\\"\\\\\\\\\\\">`\\\"foo\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_192_16_quoted_identifier_in_filt() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Quoted identifier in filter expression with spaces\",\"expression\":\"[?\\\"\\\\\\\\\\\" > `\\\"foo\\\"`]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo || bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_1_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo ||\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_2_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.|| bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_3_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\" || foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_4_foo_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo || || foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_5_foo_a_b() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[a || b]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_6_foo_a() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.[a ||]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_193_7_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"\\\"foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_0_no_key_or_value() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"No key or value\",\"error\":\"syntax\",\"expression\":\"a{}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_1_no_closing_token() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"No closing token\",\"error\":\"syntax\",\"expression\":\"a{\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_2_not_a_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not a key value pair\",\"error\":\"syntax\",\"expression\":\"a{foo}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_3_missing_value_and_closing() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value and closing character\",\"error\":\"syntax\",\"expression\":\"a{foo:\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_4_missing_closing_character() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing closing character\",\"error\":\"syntax\",\"expression\":\"a{foo: 0\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_5_missing_value() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value\",\"error\":\"syntax\",\"expression\":\"a{foo:}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_6_trailing_comma_and_no_clo() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma and no closing character\",\"error\":\"syntax\",\"expression\":\"a{foo: 0, \"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_7_missing_value_with_traili() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value with trailing comma\",\"error\":\"syntax\",\"expression\":\"a{foo: ,}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_8_accessing_array_using_an_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Accessing Array using an identifier\",\"error\":\"syntax\",\"expression\":\"a{foo: bar}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_9_a_foo_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"a{foo: 0}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_10_missing_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing key-value pair\",\"error\":\"syntax\",\"expression\":\"a.{}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_11_not_a_key_value_pair() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Not a key-value pair\",\"error\":\"syntax\",\"expression\":\"a.{foo}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_12_valid_multi_select_hash_e() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select hash extraction\",\"expression\":\"a.{foo: bar}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_13_valid_multi_select_hash_e() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select hash extraction\",\"expression\":\"a.{foo: bar, baz: bam}\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_14_trailing_comma() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma\",\"error\":\"syntax\",\"expression\":\"a.{foo: bar, }\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_15_missing_key_in_second_key() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing key in second key-value pair\",\"error\":\"syntax\",\"expression\":\"a.{foo: bar, baz}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_16_missing_value_in_second_k() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing value in second key-value pair\",\"error\":\"syntax\",\"expression\":\"a.{foo: bar, baz:}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_17_trailing_comma() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Trailing comma\",\"error\":\"syntax\",\"expression\":\"a.{foo: bar, baz: bam, }\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_18_nested_multi_select() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Nested multi select\",\"expression\":\"{\\\"\\\\\\\\\\\":{\\\" \\\":*}}\",\"result\":{\"\\\\\":{\" \":[\"object\"]}}}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_194_19_missing_closing_after_a_v() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Missing closing } after a valid nud\",\"error\":\"syntax\",\"expression\":\"{a: @\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_1_valid_multi_select_of_a_l() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a list\",\"error\":\"syntax\",\"expression\":\"foo[0, 1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_2_foo_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.[0]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_4_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma\",\"error\":\"syntax\",\"expression\":\"foo[0, ]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_5_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma and no close\",\"error\":\"syntax\",\"expression\":\"foo[0,\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_6_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with trailing comma and no close\",\"error\":\"syntax\",\"expression\":\"foo.[a\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_7_multi_select_of_a_list_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list with extra comma\",\"error\":\"syntax\",\"expression\":\"foo[0,, 1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_8_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index\",\"error\":\"syntax\",\"expression\":\"foo[abc]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_9_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using identifier indices\",\"error\":\"syntax\",\"expression\":\"foo[abc, def]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_10_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index\",\"error\":\"syntax\",\"expression\":\"foo[abc, 1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_11_multi_select_of_a_list_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a list using an identifier index with trailing comma\",\"error\":\"syntax\",\"expression\":\"foo[abc, ]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_12_valid_multi_select_of_a_h() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a hash using an identifier index\",\"expression\":\"foo.[abc]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_13_valid_multi_select_of_a_h() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Valid multi-select of a hash\",\"expression\":\"foo.[abc, def]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_14_multi_select_of_a_hash_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash using a numeric index\",\"error\":\"syntax\",\"expression\":\"foo.[abc, 1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_15_multi_select_of_a_hash_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash with a trailing comma\",\"error\":\"syntax\",\"expression\":\"foo.[abc, ]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_16_multi_select_of_a_hash_wi() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash with extra commas\",\"error\":\"syntax\",\"expression\":\"foo.[abc,, def]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_195_17_multi_select_of_a_hash_us() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"Multi-select of a hash using number indices\",\"error\":\"syntax\",\"expression\":\"foo.[0, 1]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_196_0_slice_expected_colon_or_r() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected colon or rbracket\",\"error\":\"syntax\",\"expression\":\"[:@]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_196_1_slice_has_too_many_colons() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice has too many colons\",\"error\":\"syntax\",\"expression\":\"[:::]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_196_2_slice_expected_number() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected number\",\"error\":\"syntax\",\"expression\":\"[:@:]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_196_3_slice_expected_number_of_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"slice expected number of colon\",\"error\":\"syntax\",\"expression\":\"[:1@]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_2_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"*.[0]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_3_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.[\\\"0\\\"]\",\"result\":[[null]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_4_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_5_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_6_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[#]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_197_7_missing_rbracket_for_led_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"missing rbracket for led wildcard index\",\"error\":\"syntax\",\"expression\":\"led[*\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_198_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*\",\"result\":[\"object\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_1_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_2_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.foo\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_3_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_4_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\".*\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_5_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"*foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_6_0() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"*0\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_7_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[*]bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_199_8_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[*]*\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_200_0_invalid_start_of_function() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"invalid start of function\",\"error\":\"syntax\",\"expression\":\"@(foo)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_200_1_function_names_cannot_be_() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"function names cannot be quoted\",\"error\":\"syntax\",\"expression\":\"\\\"foo\\\"(bar)\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_201_0_missing_closing_paren() {
    let case: TestCase = TestCase::from_str("{\"comment\":\"missing closing paren\",\"error\":\"syntax\",\"expression\":\"(@\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_202_0_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"![!(!\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_0_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\".\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_1_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\":\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_2_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\",\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_3_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_4_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"[\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_5_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"}\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_6_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"{\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_7_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\")\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_8_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"(\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_9_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"((&\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_10_a() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"a[\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_11_a() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"a]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_12_a() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"a][\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_203_13_() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"!\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.1\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_2_foo_11() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.-11\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_3_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_4_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\".foo\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_5_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo..bar\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_6_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo.bar.\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_syntax_204_7_foo() {
    let case: TestCase = TestCase::from_str("{\"error\":\"syntax\",\"expression\":\"foo[.]\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"type\":\"object\"}").unwrap());
    case.assert("syntax", data).unwrap();
}


#[test]
fn test_unicode_205_0_snowman() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"☃\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"☃\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_206_0_heart() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"♪♫•*¨*•.¸¸❤¸¸.•*¨*•♫♪\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"♪♫•*¨*•.¸¸❤¸¸.•*¨*•♫♪\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_207_0_yin_yang() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"\\\"☯\\\"\",\"result\":true}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"☯\":true}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_unicode_208_0_foo_ok() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[].\\\"✓\\\"\",\"result\":[\"✓\",\"✗\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"✓\":\"✓\"},{\"✓\":\"✗\"}]}").unwrap());
    case.assert("unicode", data).unwrap();
}


#[test]
fn test_wildcard_209_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*[0]\",\"result\":[0,0]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"a\":[0,1,2],\"b\":[0,1,2]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_210_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[1,2,3],\"hash\":{\"bar\":\"val\",\"foo\":\"val\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_210_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash.*\",\"result\":[\"val\",\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[1,2,3],\"hash\":{\"bar\":\"val\",\"foo\":\"val\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_210_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[1,2,3],\"hash\":{\"bar\":\"val\",\"foo\":\"val\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_210_3_array() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"array.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[1,2,3],\"hash\":{\"bar\":\"val\",\"foo\":\"val\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_210_4_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue.*\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"array\":[1,2,3],\"hash\":{\"bar\":\"val\",\"foo\":\"val\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_0_string() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_1_hash() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_2_number() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_3_nullvalue() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_4_string_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"string[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_5_hash_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"hash[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_6_number_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"number[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_7_nullvalue_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*].foo\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_211_8_nullvalue_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"nullvalue[*].foo[*].bar\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"hash\":{\"bar\":\"baz\",\"foo\":\"bar\"},\"nullvalue\":null,\"number\":23,\"string\":\"string\"}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0]\",\"result\":[[\"one\",\"two\"],[\"five\",\"six\"],[\"nine\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1]\",\"result\":[[\"three\",\"four\"],[\"seven\",\"eight\"],[\"ten\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_2_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0][0]\",\"result\":[\"one\",\"five\",\"nine\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_3_foo_1_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1][0]\",\"result\":[\"three\",\"seven\",\"ten\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_4_foo_0_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0][1]\",\"result\":[\"two\",\"six\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_5_foo_1_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1][1]\",\"result\":[\"four\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_6_foo_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_7_foo_2_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][2][2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_8_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_212_9_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"bar[*].baz[*]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[[\"one\",\"two\"],[\"three\",\"four\"]],[[\"five\",\"six\"],[\"seven\",\"eight\"]],[[\"nine\"],[\"ten\"]]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_213_0_foo_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][0]\",\"result\":[\"one\",\"three\",\"five\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\"]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_213_1_foo_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*][1]\",\"result\":[\"two\",\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[[\"one\",\"two\"],[\"three\",\"four\"],[\"five\"]]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_214_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[]},{\"bar\":[]},{\"bar\":[]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_215_0_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0]\",\"result\":[\"one\",\"three\",\"five\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_215_1_foo_bar_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[1]\",\"result\":[\"two\",\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_215_2_foo_bar_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[2]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[\"one\",\"two\"]},{\"bar\":[\"three\",\"four\"]},{\"bar\":[\"five\"]}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_216_0_foo_bar_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar.kind\",\"result\":[\"basic\",\"intermediate\",\"advanced\",\"expert\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":{\"kind\":\"basic\"}},{\"bar\":{\"kind\":\"intermediate\"}},{\"bar\":{\"kind\":\"advanced\"}},{\"bar\":{\"kind\":\"expert\"}},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_217_0_foo_bar_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[*].kind\",\"result\":[[\"basic\",\"intermediate\"],[\"advanced\",\"expert\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"kind\":\"basic\"},{\"kind\":\"intermediate\"}]},{\"bar\":[{\"kind\":\"advanced\"},{\"kind\":\"expert\"}]},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_217_1_foo_bar_0_kind() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar[0].kind\",\"result\":[\"basic\",\"advanced\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":[{\"kind\":\"basic\"},{\"kind\":\"intermediate\"}]},{\"bar\":[{\"kind\":\"advanced\"},{\"kind\":\"expert\"}]},{\"bar\":\"string\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*]\",\"result\":[[\"one\",\"two\"],[\"three\",\"four\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_1_foo_bar_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0]\",\"result\":[\"one\",\"two\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_2_foo_bar_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0]\",\"result\":\"one\"}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_3_foo_bar_0_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_4_foo_bar_0_0_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[0][0][0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_218_5_foo_0_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[0][0]\",\"result\":null}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[[\"one\",\"two\"],[\"three\",\"four\"]]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_219_0_foo_bar_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz\",\"result\":[[\"one\",\"two\",\"three\"],[\"four\",\"five\",\"six\"],[\"seven\",\"eight\",\"nine\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_219_1_foo_bar_baz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[0]\",\"result\":[\"one\",\"four\",\"seven\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_219_2_foo_bar_baz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[1]\",\"result\":[\"two\",\"five\",\"eight\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_219_3_foo_bar_baz_2() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[2]\",\"result\":[\"three\",\"six\",\"nine\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_219_4_foo_bar_baz_3() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar[*].baz[3]\",\"result\":[]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":[{\"baz\":[\"one\",\"two\",\"three\"]},{\"baz\":[\"four\",\"five\",\"six\"]},{\"baz\":[\"seven\",\"eight\",\"nine\"]}]}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_220_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*]\",\"result\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_220_1_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_220_2_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"[*].notbar\",\"result\":[\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_221_0_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].bar\",\"result\":[\"one\",\"two\",\"three\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_221_1_foo_notbar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo[*].notbar\",\"result\":[\"four\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":[{\"bar\":\"one\"},{\"bar\":\"two\"},{\"bar\":\"three\"},{\"notbar\":\"four\"}]}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_222_0_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*\",\"result\":[{\"sub1\":{\"foo\":\"one\"}},{\"sub1\":{\"foo\":\"one\"}}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_222_1_sub1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.sub1\",\"result\":[{\"foo\":\"one\"},{\"foo\":\"one\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_222_2_() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*\",\"result\":[[{\"foo\":\"one\"}],[{\"foo\":\"one\"}]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_222_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.*.foo[]\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_222_4_sub1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.sub1.foo\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"top1\":{\"sub1\":{\"foo\":\"one\"}},\"top2\":{\"sub1\":{\"foo\":\"one\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_223_0_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"*.bar\",\"result\":[\"one\",\"one\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":\"one\"},\"nomatch\":{\"notbar\":\"three\"},\"other\":{\"bar\":\"one\"}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_224_0_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*\",\"result\":[{\"second-1\":\"val\"},{\"second-1\":\"val\"},{\"second-1\":\"val\"}]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_224_1_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*\",\"result\":[[\"val\"],[\"val\"],[\"val\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_224_2_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*.*\",\"result\":[[],[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_224_3_foo() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.*.*.*\",\"result\":[[],[],[]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"first-1\":{\"second-1\":\"val\"},\"first-2\":{\"second-1\":\"val\"},\"first-3\":{\"second-1\":\"val\"}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_225_0_foo_baz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.baz\",\"result\":[\"val\",\"val\",\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_225_1_foo_bar() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.bar.*\",\"result\":[\"val\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_225_2_foo_notbaz() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz\",\"result\":[[\"a\",\"b\",\"c\"],[\"a\",\"b\",\"c\"]]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_225_3_foo_notbaz_0() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz[0]\",\"result\":[\"a\",\"a\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}


#[test]
fn test_wildcard_225_4_foo_notbaz_1() {
    let case: TestCase = TestCase::from_str("{\"expression\":\"foo.*.notbaz[-1]\",\"result\":[\"c\",\"c\"]}").unwrap();
    let data = Rcvar::new(Variable::from_json("{\"foo\":{\"bar\":{\"baz\":\"val\"},\"other\":{\"baz\":\"val\"},\"other2\":{\"baz\":\"val\"},\"other3\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other4\":{\"notbaz\":[\"a\",\"b\",\"c\"]},\"other5\":{\"other\":{\"a\":1,\"b\":1,\"c\":1}}}}").unwrap());
    case.assert("wildcard", data).unwrap();
}

