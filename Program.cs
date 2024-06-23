using Microsoft.Extensions.DependencyInjection;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.Markup.Declarative;
using Avalonia.Media.Imaging;
using Avalonia.Platform;
using FlatboxEditor.UI.View;
using FlatboxEditor.UI.Data;

namespace FlatboxEditor;

class Program
{
    public static void Main(string[] args) {
        var lifetime = new ClassicDesktopStyleApplicationLifetime { Args = args, ShutdownMode = ShutdownMode.OnLastWindowClose };

        AppBuilder.Configure<App>()
            .UsePlatformDetect()
            .UseServiceProvider(
                new ServiceCollection()
                    .AddSingleton<EditorData>()
                    .BuildServiceProvider()
            )
            .SetupWithLifetime(lifetime);

        lifetime.MainWindow = new Window()
            .Title("Flatbox Editor")
            .Width(800)
            .Height(450)
            .WindowState(WindowState.Maximized)
            .Icon(new WindowIcon(new Bitmap(
                AssetLoader.Open(new Uri("avares://FlatboxEditor/Assets/icons/favicon.ico"))
            )))
            .Content(new EditorView());

        #if DEBUG
        lifetime.MainWindow.AttachDevTools();
        #endif

        lifetime.Start(args);
    }
}
