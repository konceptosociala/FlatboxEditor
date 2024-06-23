using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Declarative;
using Avalonia.Media;

namespace FlatboxEditor.UI.Components;

public class FlatButton(string iconName, Action<RoutedEventArgs>? onClick = null) : ComponentBase
{
    private readonly string _iconName = iconName;
    private readonly Action<RoutedEventArgs>? _onClick = onClick;

    protected override object Build()
    {
        return new Button()
            .Classes("FlatButton")
            .Height(64)
            .Width(64)
            .OnClick(_onClick ?? ((e) => {}))
            .Content(
                new PathIcon()
                    .Width(32)
                    .Height(32)
                    .Data(App.GetResource<StreamGeometry>(_iconName)!)
                    .Foreground(new SolidColorBrush(App.GetResource<Color>("SystemAccentColor")!))
            );
    }
}