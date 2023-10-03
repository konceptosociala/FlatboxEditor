using FlatboxEditor.FFI;
using ReactiveUI;

namespace FlatboxEditor.MVVM.ViewModels;

public class MainWindowViewModel : ViewModelBase 
{
    private ViewModelBase _contentViewModel;

    public ViewModelBase ContentViewModel {
        get => _contentViewModel;
        private set => this.RaiseAndSetIfChanged(ref _contentViewModel, value);
    }

    public EditorViewModel Editor;

    public MainWindowViewModel(){
        Editor = new EditorViewModel();
        _contentViewModel = Editor;
    }
}
