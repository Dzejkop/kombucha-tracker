INSERT INTO kombucha (name, added)
VALUES
    ('Banana #1 brew', NOW()),
    ('Banana #2 brew', NOW());

INSERT INTO kombucha_entry (kombucha_id, content, added)
VALUES
    (1, 'Started F1', NOW()),
    (2, 'Started F1', NOW());

INSERT INTO kombucha (name, added)
VALUES
    ('Banana #1 brew', NOW());

INSERT INTO kombucha_fermentation ( kombucha_id, start_date, status )
VALUES
    (1, NOW(), 'primary');