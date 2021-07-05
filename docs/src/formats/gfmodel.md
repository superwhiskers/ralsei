# gfmodel ("girlfriend model")

with pokemon sun and moon, instead of sticking with bch (the model format used by pokemon x and y,)
game freak decided to create a new format, called `gfmodel` (likely standing for "game freak
model.") seemingly, there is no comprehensive text documentation of this format, so this document
seeks to provide a comprehensive overview of the format's structure along with additional
information about it.

## prior art

as far as i know, the only source of information regarding the layout of this format comes from
[ohana3ds], which is only source code---no actual documentation. however, this remained pretty much
the only source of information necessary to compose this document, so it is listed here

aside from that, some additional information with regard to the history/creation of the format has
been sourced from [this reddit post]

[ohana3ds]: https://github.com/gdkchan/Ohana3DS-Rebirth/blob/master/Ohana3DS%20Rebirth/Ohana/Models/PocketMonsters/GfModel.cs
[this reddit post]: https://reddit.com/cgh8fy

## overview

a gfmodel is structured like this

+------------+----------+-----------+----------------+--------------------------------------+
| offset     | variable | size      | type           | description                          |
+============+==========+===========+================+======================================+
| 0x00       |          | 0x04      | u32            | the magic, `0x00010000`              |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x04       | M        | 0x04      | u32            | the number of models                 |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x08       | T        | 0x04      | u32            | the number of textures               |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x0c       | U1       | 0x04      | u32            | the number of items in the first     |
|            |          |           |                | unknown section                      |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x10       | U2       | 0x04      | u32            | the number of items in the second    |
|            |          |           |                | unknown section                      |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x14       | U3       | 0x04      | u32            | the number of items in the third     |
|            |          |           |                | unknown section                      |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x18       |          | 0x04 * M  | array[u32][M]  | the offsets of each model pointer in |
|            |          |           |                | the gfmodel                          |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x18 +     |          | 0x04 * T  | array[u32][T]  | the offsets of each texture pointer  |
| (0x04 * M) |          |           |                | in the gfmodel                       |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x18 +     |          | 0x04 * U1 | array[u32][U1] | the offsets of each item pointer in  |
| (0x04 *    |          |           |                | the first unknown section of the     |
| (M + T))   |          |           |                | gfmodel                              |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x18 +     |          | 0x04 * U2 | array[u32][U2] | the offsets of each item pointer in  |
| (0x04 *    |          |           |                | the second unknown section of the    |
| (M + T +   |          |           |                | gfmodel                              |
| U1))       |          |           |                |                                      |
+------------+----------+-----------+----------------+--------------------------------------+
| 0x18 +     |          | 0x04 * U3 | array[u32][U3] | the offsets of each item pointer in  |
| (0x04 *    |          |           |                | the third unknown section of the     |
| (M + T +   |          |           |                | gfmodel                              |
| U1 + U2))  |          |           |                |                                      |
+------------+----------+-----------+----------------+--------------------------------------+
| \ldots     |          |           |                | the different parts of the gfmodel   |
+------------+----------+-----------+----------------+--------------------------------------+

## pointer structure

instead of pointing to the model/texture/item directly, the offsets in the gfmodel's "header" point
to a "pointer" containing the name of the wanted item, which then have another address pointing to
the actual structure itself. here is how they are structured

+--------+----------+------+--------+-------------------------------------------------------+
| offset | variable | size | type   | description                                           |
+========+==========+======+========+=======================================================+
| 0x00   | L        | 0x01 | u8     | the length of the name string                         |
+--------+----------+------+--------+-------------------------------------------------------+
| 0x01   |          | L    | str[L] | the name of the item being pointed to                 |
+--------+----------+------+--------+-------------------------------------------------------+
| 0x01 + |          | 0x04 | u32    | the data's offset in the gfmodel                      |
| L      |          |      |        |                                                       |
+--------+----------+------+--------+-------------------------------------------------------+

## model structure



relevant content:

- [`OModel`{.cs}](https://github.com/gdkchan/Ohana3DS-Rebirth/blob/76e3976b202ec117d47cbc3b476fc4d540a678c0/Ohana3DS%20Rebirth/Ohana/RenderBase.cs#L1490)
- [`GfModel.loadModel()`{.cs}](https://github.com/gdkchan/Ohana3DS-Rebirth/blob/master/Ohana3DS%20Rebirth/Ohana/Models/PocketMonsters/GfModel.cs#L85)
