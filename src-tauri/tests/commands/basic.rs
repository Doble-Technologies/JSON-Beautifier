use json_beauty_lib::json_beauty;

#[test]
fn json_beauty_01() {
    assert_eq!(json_beauty("{\"name\":\"John\",\"age\":25,\"active\":true}"), "{\n  \"active\": true,\n  \"age\": 25,\n  \"name\": \"John\"\n}");
}

#[test]
fn json_beauty_02() {
    assert_eq!(json_beauty("[{\"id\": 1, \"status\": \"ok\"}, null, 42, \"text\", false]"), "[\n  {\n    \"id\": 1,\n    \"status\": \"ok\"\n  },\n  null,\n  42,\n  \"text\",\n  false\n]");
}

#[test]
fn json_beauty_fail_01() {
    assert_eq!(json_beauty(""), "Err: EOF while parsing a value at line 1 column 0");
}