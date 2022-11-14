CREATE TABLE IF NOT EXISTS yars.Posts
(
    PostID     INT unsigned NOT NULL AUTO_INCREMENT,
    CreateDate DATETIME DEFAULT CURRENT_TIMESTAMP,
    Title      VARCHAR(200) NOT NULL,
    BodyID     INT unsigned NOT NULL,
    PRIMARY KEY (PostID),
    FOREIGN KEY (BodyID)
        REFERENCES yars.PostBody (BodyID)
        ON DELETE CASCADE
);


