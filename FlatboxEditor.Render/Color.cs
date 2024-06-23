namespace FlatboxEditor.Render;

public struct Color(byte r, byte g, byte b)
{
    public byte r = r;
    public byte g = g;
    public byte b = b;

    public static Color Light() => new(224, 224, 224);

    public static Color Dark() => new(18, 18, 18);

    public static Color Red() => new(255, 0, 63);

    public static Color Orange() => new(253, 86, 54);

    public static Color Yellow() => new(255, 200, 85);

    public static Color Lime() => new(147, 238, 115);

    public static Color Green() => new(0, 214, 137);

    public static Color Magenta() => new(127, 0, 95);

    public static Color Blue() => new(32, 0, 121);
    
}