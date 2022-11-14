DROP PROCEDURE IF EXISTS yars.GetPosts;
CREATE PROCEDURE yars.GetPosts()
BEGIN
    SELECT p.postid, p.createdate, p.title
    FROM posts p
    ORDER BY p.postid DESC;
END;


