To render a block, we need it's:
- Position
- Texture (index)
- UV coordinates
- Ambient occlusion value

We need this data **for each vertex**

If we were to store all of this separately, we would need 12+4+8+4=26 bytes **per vertex**

These can be condensed quite a bit.
- Position (15)
    5 bits per axis, 15 bits for XYZ
- Texture index (8)
    For 1.7.10, we can just index into a 16*16 texture atlas
    8 bits to store block index
- UV coordinates (2)
    Since blocks only have 4 vertices, we can create a lookup table and use 2 bits per vertex
- Ambient occlusion (2)
    Same as UV coordinates, we only have 4 different values, so 2 bits is enough
- Light value (4)
    And for the remaining bytes we can use the lighting value, for which we have 5 bits left, one more than we actually need!

Coming in at a total of 27 bits! Only 1 32 bit integer per block, that's 6.5 times as efficient!