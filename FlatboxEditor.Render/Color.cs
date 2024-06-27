namespace FlatboxEditor.Render;

public struct Color(byte r, byte g, byte b)
{
    public byte r = r;
    public byte g = g;
    public byte b = b;

    public static readonly Color LIGHT = new(224, 224, 224);

    public static readonly Color DARK = new(18, 18, 18);

    public static readonly Color RED = new(255, 0, 63);

    public static readonly Color ORANGE = new(253, 86, 54);

    public static readonly Color YELLOW = new(255, 200, 85);

    public static readonly Color LIME = new(147, 238, 115);

    public static readonly Color GREEN = new(0, 214, 137);

    public static readonly Color MAGENTA = new(127, 0, 95);

    public static readonly Color BLUE = new(32, 0, 121);
    
}