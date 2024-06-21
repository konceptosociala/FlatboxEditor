using System;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.ApplicationLifetimes;
using Avalonia.ReactiveUI;
using Avalonia.Themes.Fluent;
using Avalonia.Markup.Declarative;

namespace FlatboxEditor;

class Program
{
    public static void Main(string[] args) {
        var lifetime = new ClassicDesktopStyleApplicationLifetime { Args = args, ShutdownMode = ShutdownMode.OnLastWindowClose };

        AppBuilder.Configure<Application>()
            .UsePlatformDetect()
            .AfterSetup(b => b.Instance?.Styles.Add(new FluentTheme()))
            .SetupWithLifetime(lifetime);

        lifetime.MainWindow = new Window()
            .Title("Flatbox Editor");
    }
}

