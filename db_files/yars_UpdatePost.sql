DROP PROCEDURE IF EXISTS yars.UpdatePost;
CREATE PROCEDURE yars.UpdatePost(post_id INT, updated_body TEXT)
BEGIN

    SELECT @target_body_id := BodyID FROM Posts WHERE PostID = post_id;
    UPDATE PostBody
        SET BodyText = updated_body
    WHERE BodyID = @target_body_id;

END;

