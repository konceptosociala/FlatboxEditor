#pragma warning disable CS8600

using Avalonia;
using Avalonia.Markup.Xaml;

namespace FlatboxEditor;

public partial class App : Application
{
    public override void Initialize()
    {
        AvaloniaXamlLoader.Load(this);
    }

    public static T? GetResource<T>(object key)
    {
        if (Application.Current!.TryGetResource(key, Application.Current!.ActualThemeVariant, out object resource))
        {
            if (resource is T)
            {
                return (T) resource;
            }
        }

        return default;
    }
}