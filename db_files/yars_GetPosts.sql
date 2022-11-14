DROP PROCEDURE IF EXISTS yars.GetPosts;
CREATE PROCEDURE yars.GetPosts()
BEGIN
    SELECT p.postid, p.createdate, p.title, p.BodyID
    FROM posts p
    ORDER BY p.postid DESC;
END;


