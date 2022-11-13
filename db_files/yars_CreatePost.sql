DROP PROCEDURE IF EXISTS yars.CreatePost;
CREATE PROCEDURE yars.CreatePost(new_title VARCHAR(200), body TEXT)
BEGIN
    INSERT INTO PostBody (BodyText)
    VALUES (body);
    SET @body_id = LAST_INSERT_ID();

    INSERT INTO Posts (Title, BodyID) VALUES (new_title, @body_id);
END;


