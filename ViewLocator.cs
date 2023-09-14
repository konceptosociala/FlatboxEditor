using System;
using Avalonia.Controls;
using Avalonia.Controls.Templates;
using SonjaEditor.MVVM.ViewModels;

namespace SonjaEditor;

public class ViewLocator : IDataTemplate
{
    public Control Build(object? data) {      
        var name = data!.GetType().FullName!.Replace("ViewModel", "View");
        var type = Type.GetType(name);

        if (type == null)
            return new TextBlock { Text = "Not Found: " + name };
            
        return (Control)Activator.CreateInstance(type)!;       
    }

    public bool Match(object? data) {
        return data is ViewModelBase;
    }
}