namespace FlatboxEditor.Render;

public struct Color {
    public byte r;
    public byte g;
    public byte b;

    public Color(byte r, byte g, byte b) {
        this.r = r;
        this.g = g;
        this.b = b;
    }

    public static Color Light() => new Color(224, 224, 224);

    public static Color Dark() => new Color(18, 18, 18);

    public static Color Red() => new Color(255, 0, 63);

    public static Color Orange() => new Color(253, 86, 54);

    public static Color Yellow() => new Color(255, 200, 85);

    public static Color Lime() => new Color(147, 238, 115);

    public static Color Green() => new Color(0, 214, 137);

    public static Color Magenta() => new Color(127, 0, 95);

    public static Color Blue() => new Color(32, 0, 121);
    
}