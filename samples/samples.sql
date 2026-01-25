INSERT INTO
    users (
        login,
        username,
        salt,
        password_hash,
        sire
    )
VALUES (
        'alex',
        'Alex',
        'BriUIvYDXESWprriWyVsvMsaEqY3aDhK',
        'IZdudvRoOmgxtA3eEAKznjO06p3XIj9iOtRvEp1Kv6ZNiKcuKJH9Gvktp/uCQnPNn78Ui8KtZZVt4j6Qo2ferA==', -- qwerty123
        NULL
    );

INSERT INTO
    entries (
        id,
        name,
        image,
        description,
        note,
        created_at,
        stored_in,
        responsible_person
    )
VALUES (
        'USBFLASH-001-AF3',
        'USB Flash Drive',
        'B+MYRqOS126+0gHPAA08xFwbWVDBl8HH1WauYpzO1UBE1RpPG2yWsQuyzneZ3LPhW51w9GviHUsMs79ZCpuhEg==',
        '16 GB',
        NULL,
        datetime('now'),
        NULL,
        'alex'
    );

INSERT INTO
    images (id, data)
VALUES (
        'B+MYRqOS126+0gHPAA08xFwbWVDBl8HH1WauYpzO1UBE1RpPG2yWsQuyzneZ3LPhW51w9GviHUsMs79ZCpuhEg==',
        readfile ('samples/usb-drive.png')
    );

INSERT INTO
    entries (
        id,
        name,
        image,
        description,
        note,
        created_at,
        stored_in,
        responsible_person
    )
VALUES (
        'DISPLAY-014-E57',
        'Lenovo 14-inch Display',
        NULL,
        '1920x1080',
        NULL,
        datetime('now'),
        NULL,
        'alex'
    );

INSERT INTO
    entries (
        id,
        name,
        image,
        description,
        note,
        created_at,
        stored_in,
        responsible_person
    )
VALUES (
        'SCREWDRI-993-000',
        'Screwdriver',
        NULL,
        NULL,
        NULL,
        datetime('now'),
        NULL,
        'alex'
    );