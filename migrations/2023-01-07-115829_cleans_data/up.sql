DELETE FROM products
WHERE vid IN (SELECT vid FROM 
(SELECT vid, 
ROW_NUMBER() OVER(PARTITION BY id
ORDER BY id ASC) AS row_num
FROM products) pro
WHERE pro.row_num > 1 );

DELETE FROM comments
WHERE vid IN (SELECT vid FROM 
(SELECT vid, 
ROW_NUMBER() OVER(PARTITION BY id
ORDER BY id ASC) AS row_num
FROM comments) com
WHERE com.row_num > 1 );
