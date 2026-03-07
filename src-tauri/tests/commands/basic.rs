use json_beauty_lib::{json_beauty, xml_beauty};

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

#[test]
fn xml_beauty_01() {
    let test_str= r#"<?xml version="1.0" encoding="UTF-8"?><bookstore><book category="fiction"><title lang="en">The Great Gatsby</title><author>F. Scott Fitzgerald</author><year>1925</year><price>12.99</price></book><book category="nonfiction"><title lang="en">Sapiens</title><author>Yuval Noah Harari</author><year>2011</year><price>15.99</price></book></bookstore>"#;
    let ans= xml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"<?xml version="1.0" encoding="UTF-8"?>
<bookstore>
  <book category="fiction">
    <title lang="en">The Great Gatsby</title>
    <author>F. Scott Fitzgerald</author>
    <year>1925</year>
    <price>12.99</price>
  </book>
  <book category="nonfiction">
    <title lang="en">Sapiens</title>
    <author>Yuval Noah Harari</author>
    <year>2011</year>
    <price>15.99</price>
  </book>
</bookstore>"#;
    assert_eq!(ans,expected);
}

#[test]
fn xml_beauty_02() {
    let test_str= r#"<?xml version="1.0" encoding="UTF-8"?><project xmlns="http://maven.apache.org/POM/4.0.0"><modelVersion>4.0.0</modelVersion><groupId>com.example</groupId><artifactId>my-app</artifactId><version>1.0.0</version><dependencies><dependency><groupId>org.springframework.boot</groupId><artifactId>spring-boot-starter-web</artifactId><version>3.2.0</version></dependency></dependencies></project>"#;
    let ans= xml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"<?xml version="1.0" encoding="UTF-8"?>
<project xmlns="http://maven.apache.org/POM/4.0.0">
  <modelVersion>4.0.0</modelVersion>
  <groupId>com.example</groupId>
  <artifactId>my-app</artifactId>
  <version>1.0.0</version>
  <dependencies>
    <dependency>
      <groupId>org.springframework.boot</groupId>
      <artifactId>spring-boot-starter-web</artifactId>
      <version>3.2.0</version>
    </dependency>
  </dependencies>
</project>"#;
    assert_eq!(ans,expected);
}

#[test]
fn xml_beauty_03() {
    let test_str= r#""#;
    let ans= xml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#""#;
    assert_eq!(ans,expected);
}

#[test]
fn xml_beauty_fail_01() {
    let test_str= r#"<?xmlsa version="1.0" encoding="UTF-8"?>bookstore><book category="fiction"><title lang="en"The Great Gatsby</title><author>F. Scott Fitzgerald</author><year>1925</year><price>12.99</price></book><book category="nonfiction"><title lang="en">Sapiens</title><author>Yuval Noah Harari</author><year>2011</year><price>15.99</price></book></bookstore>"#;
    let ans= xml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"ill-formed document: expected `</title>`, but `</book>` was found"#;
    assert_eq!(ans,expected);
}