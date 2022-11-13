DROP PROCEDURE IF EXISTS yars.DeletePost;
CREATE PROCEDURE yars.DeletePost(post_id INT)
BEGIN

    SELECT @target_body_id := BodyID FROM Posts WHERE PostID = post_id;

    DELETE
    FROM PostBody
    WHERE BodyID = @target_body_id;

    DELETE
    FROM Posts
    WHERE PostID = post_id;

END;