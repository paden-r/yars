DROP PROCEDURE IF EXISTS yars.GetPostBody;
CREATE PROCEDURE yars.GetPostBody(post_id INT)
BEGIN
    SELECT p.postid, p.createdate, p.title, pb.bodytext
    FROM Posts p
             INNER JOIN PostBody pb ON p.bodyid = pb.bodyid
    WHERE p.postid = post_id;
END;
