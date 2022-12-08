DROP PROCEDURE IF EXISTS yars.CreatePost;
CREATE PROCEDURE CreatePost(IN new_title varchar(200),IN body text,IN summary varchar(200),IN ranking varchar(50))
BEGIN
    INSERT INTO PostBody (BodyText)
    VALUES (body);
    SET @body_id = LAST_INSERT_ID();

    INSERT INTO Posts (Title, BodyID, Summary, Ranking) VALUES (new_title, @body_id, summary, ranking);
END
