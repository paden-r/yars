CREATE TABLE IF NOT EXISTS yars.Posts
(
    PostID     INT unsigned NOT NULL AUTO_INCREMENT,
    CreateDate DATETIME DEFAULT CURRENT_TIMESTAMP,
    Title      VARCHAR(200) NOT NULL,
    Summary    VARCHAR(200),
    Ranking       VARCHAR(50),
    BodyID     INT unsigned NOT NULL,
    PRIMARY KEY (PostID),
    FOREIGN KEY (BodyID)
        REFERENCES yars.PostBody (BodyID)
        ON DELETE CASCADE
);
