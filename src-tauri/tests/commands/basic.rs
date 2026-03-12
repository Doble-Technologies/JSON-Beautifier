use json_beauty_lib::{json_beauty, md_beauty, sql_beauty, xml_beauty, yml_beauty};

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
    assert_eq!(json_beauty("abc"), "abc");
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



#[test]
fn sql_beauty_01(){
    let test_str= r#"select u.id,u.first_name,u.last_name,u.email,u.created_at,p.plan_name,p.price,o.id as order_id,o.total,o.status,o.created_at as order_date from users u left join subscriptions s on s.user_id=u.id left join plans p on p.id=s.plan_id left join orders o on o.user_id=u.id where u.created_at>='2024-01-01' and u.is_deleted=0 and(o.status='completed' or o.status='pending') order by u.created_at desc,o.total desc;insert into audit_log(user_id,action,metadata,created_at) select u.id,'subscription_review',json_object('plan',p.plan_name,'orders_count',count(o.id),'total_spent',sum(o.total)),now() from users u join subscriptions s on s.user_id=u.id join plans p on p.id=s.plan_id left join orders o on o.user_id=u.id and o.status='completed' where s.expires_at between now() and date_add(now(),interval 30 day) group by u.id,p.plan_name having sum(o.total)>500;update users u join(select user_id,count(*) as cnt from orders where status='completed' and created_at>=date_sub(now(),interval 90 day) group by user_id having cnt>=3) as loyal on loyal.user_id=u.id set u.tier='gold',u.updated_at=now() where u.tier!='gold';"#;
    let ans= sql_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"SELECT
  u.id,
  u.first_name,
  u.last_name,
  u.email,
  u.created_at,
  p.plan_name,
  p.price,
  o.id AS order_id,
  o.total,
  o.status,
  o.created_at AS order_date
FROM
  users u
  LEFT JOIN subscriptions s ON s.user_id = u.id
  LEFT JOIN plans p ON p.id = s.plan_id
  LEFT JOIN orders o ON o.user_id = u.id
WHERE
  u.created_at >= '2024-01-01'
  AND u.is_deleted = 0
  AND(
    o.status = 'completed'
    OR o.status = 'pending'
  )
ORDER BY
  u.created_at DESC,
  o.total DESC;
INSERT INTO
  audit_log(user_id, ACTION, metadata, created_at)
SELECT
  u.id,
  'subscription_review',
  json_object(
    'plan',
    p.plan_name,
    'orders_count',
    count(o.id),
    'total_spent',
    sum(o.total)
  ),
  NOW()
FROM
  users u
  JOIN subscriptions s ON s.user_id = u.id
  JOIN plans p ON p.id = s.plan_id
  LEFT JOIN orders o ON o.user_id = u.id
  AND o.status = 'completed'
WHERE
  s.expires_at BETWEEN NOW()
  AND date_add(NOW(), INTERVAL 30 DAY)
GROUP BY
  u.id,
  p.plan_name
HAVING
  sum(o.total) > 500;
UPDATE
  users u
  JOIN(
    SELECT
      user_id,
      count(*) AS cnt
    FROM
      orders
    WHERE
      STATUS = 'completed'
      AND created_at >= date_sub(NOW(), INTERVAL 90 DAY)
    GROUP BY
      user_id
    HAVING
      cnt >= 3
  ) AS loyal ON loyal.user_id = u.id
SET
  u.tier = 'gold',
  u.updated_at = NOW()
WHERE
  u.tier != 'gold';"#;
    assert_eq!(ans,expected);
}

#[test]
fn sql_beauty_02(){
    let test_str= r#"select department,employee_id,first_name,last_name,salary,rank() over(partition by department order by salary desc) as dept_rank,round(salary/avg(salary) over(partition by department)*100,2) as pct_of_dept_avg,sum(salary) over(partition by department) as dept_payroll,count(*) over(partition by department) as dept_headcount,sum(case when performance_rating='exceeds' then 1 else 0 end) over(partition by department) as high_performers,lag(salary,1) over(partition by department order by hire_date) as prev_hire_salary from employees where termination_date is null and hire_date<curdate() having dept_rank<=5 order by department,dept_rank;"#;
    let ans= sql_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"SELECT
  department,
  employee_id,
  first_name,
  last_name,
  salary,
  rank() over(
    PARTITION by department
    ORDER BY
      salary DESC
  ) AS dept_rank,
  round(
    salary / avg(salary) over(PARTITION by department) * 100,
    2
  ) AS pct_of_dept_avg,
  sum(salary) over(PARTITION by department) AS dept_payroll,
  count(*) over(PARTITION by department) AS dept_headcount,
  sum(
    CASE
      WHEN performance_rating = 'exceeds' THEN 1
      ELSE 0
    END
  ) over(PARTITION by department) AS high_performers,
  lag(salary, 1) over(
    PARTITION by department
    ORDER BY
      hire_date
  ) AS prev_hire_salary
FROM
  employees
WHERE
  termination_date IS NULL
  AND hire_date < curdate()
HAVING
  dept_rank <= 5
ORDER BY
  department,
  dept_rank;"#;
    assert_eq!(ans,expected);
}

#[test]
fn sql_beauty_03(){
    let test_str= r#"with monthly_revenue as(select date_format(created_at,'%Y-%m') as month,sum(total) as revenue,count(*) as order_count from orders where created_at>=date_sub(now(),interval 12 month) and status!='refunded' group by date_format(created_at,'%Y-%m')),top_products as(select p.id,p.name,p.category,sum(oi.quantity) as units_sold,sum(oi.quantity*oi.unit_price) as revenue from products p join order_items oi on oi.product_id=p.id join orders o on o.id=oi.order_id where o.status='completed' group by p.id,p.name,p.category having sum(oi.quantity)>100 order by revenue desc limit 10) select mr.month,mr.revenue,mr.order_count,tp.name as top_product,tp.units_sold from monthly_revenue mr cross join top_products tp where tp.revenue=(select max(revenue) from top_products) order by mr.month asc;"#;
    let ans= sql_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"WITH monthly_revenue AS(
  SELECT
    date_format(created_at, '%Y-%m') AS MONTH,
    sum(total) AS revenue,
    count(*) AS order_count
  FROM
    orders
  WHERE
    created_at >= date_sub(NOW(), INTERVAL 12 MONTH)
    AND STATUS != 'refunded'
  GROUP BY
    date_format(created_at, '%Y-%m')
),
top_products AS(
  SELECT
    p.id,
    p.name,
    p.category,
    sum(oi.quantity) AS units_sold,
    sum(oi.quantity * oi.unit_price) AS revenue
  FROM
    products p
    JOIN order_items oi ON oi.product_id = p.id
    JOIN orders o ON o.id = oi.order_id
  WHERE
    o.status = 'completed'
  GROUP BY
    p.id,
    p.name,
    p.category
  HAVING
    sum(oi.quantity) > 100
  ORDER BY
    revenue DESC
  LIMIT
    10
)
SELECT
  mr.month,
  mr.revenue,
  mr.order_count,
  tp.name AS top_product,
  tp.units_sold
FROM
  monthly_revenue mr
  CROSS JOIN top_products tp
WHERE
  tp.revenue =(
    SELECT
      max(revenue)
    FROM
      top_products
  )
ORDER BY
  mr.month ASC;"#;
    assert_eq!(ans,expected);
}
#[test]
fn sql_beauty_invalid_01(){
    let test_str= r#"selec user_id,, email, form users whre users.active=1 and created_at beween '2024-01-01' and '2024-12-31' joyn orders on orders.user_id=users.id groupp by user_id having count(orders.id)>"#;
    let ans= sql_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"selec user_id,
,
email,
form users whre users.active = 1
AND created_at beween '2024-01-01'
AND '2024-12-31' joyn orders ON orders.user_id = users.id groupp by user_id
HAVING
  count(orders.id) >"#;
    assert_eq!(ans,expected);
}

#[test]
fn sql_beauty_empty_01() {
    let test_str= r#""#;
    let ans= sql_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#""#;
    assert_eq!(ans,expected);
}

#[test]
fn yml_beauty_01() {
    let test_str= r#"name: John Doe
age: 30
email: john.doe@example.com
active: true
roles: [admin, user]
address: {street: 123 Main St, city: Springfield, zip: "12345"}"#;
    let ans= yml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= "name: John Doe\nage: 30\nemail: john.doe@example.com\nactive: true\nroles:\n- admin\n- user\naddress:\n  street: 123 Main St\n  city: Springfield\n  zip: '12345'\n";
    assert_eq!(ans,expected);
}

#[test]
fn yml_beauty_fail_01() {
    let test_str= r#"app:name: "my-service"
version: "2.4.1"
environment:   production
server:  host:    "0.0.0.0"
  port: 8080   ssl: enabled: true    cert: "/etc/ssl/certs/server.crt"    key: "/etc/ssl/private/server.key"
database:  primary:   host: "db-primary.internal"    port: 5432  name: "appdb"    pool:  min: 2      max:    20   timeout: 30000    replica:  host: "db-replica.internal"  port: 5432
cache: host: "redis.internal"  port: 6379   ttl: 3600  max_memory: "512mb"
features:  dark_mode: true  beta_signup:    false  analytics: true   new_dashboard: false"#;
    let ans= yml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#"Failed to parse YAML: mapping values are not allowed in this context at line 4 column 14"#;
    assert_eq!(ans,expected);
}

#[test]
fn yml_beauty_empty_01() {
    let test_str= r#""#;
    let ans= yml_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= "null\n";
    assert_eq!(ans,expected);
}

#[test]
fn md_beauty_01(){
    let test_str = r"# Heading\n\n|Name|Age|\n|---|---|\n|Alice|30|";
    let ans= md_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= "# Heading\n\n| Name  | Age |\n| ----- | --- |\n| Alice | 30  |";
    assert_eq!(ans,expected);
}

#[test]
fn md_beauty_02(){
    let test_str = r"# API Docs\n## Endpoints\n\GET /users` returns all users.\n`POST /users` creates a user.\n\n> Note: auth required`";
    let ans= md_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= "# API Docs\n\n## Endpoints\n\n\\\\GET /users`returns all users.`POST /users\\` creates a user.\n\n > \n > Note: auth required\\`";
    assert_eq!(ans,expected);
}

#[test]
fn md_beauty_03(){
    let test_str = "###QuickNotes
|Key|Value|
|a|1|
|b|2|
|c|3|
Final paragraph without spacing ending abruptly.";
    let ans= md_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= "### QuickNotes\n\n| \\   | Key | Value |\n| \\   | a   | 1     |\n| \\   | b   | 2     |\n| \\   | c   | 3     |\nFinal paragraph without spacing ending abruptly.";
    assert_eq!(ans,expected);
}

#[test]
fn md_beauty_fail_01(){
    let test_str = "";
    let ans= md_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#""#;
    assert_eq!(ans,expected);
}
#[test]
fn md_beauty_empty_01(){
    let test_str= r#""#;
    let ans= md_beauty(test_str).unwrap_or_else(|e| e.to_string());
    let expected= r#""#;
    assert_eq!(ans,expected);
}