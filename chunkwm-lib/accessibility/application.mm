#include "../../chunkwm/src/common/misc/carbon.cpp"
#include "../../chunkwm/src/common/misc/workspace.mm"
#include "../../chunkwm/src/common/accessibility/application.cpp"

extern "C" macos_application *axlib_construct_focused_application() {
    return AXLibConstructFocusedApplication();
};
extern "C" macos_application *axlib_construct_application(ProcessSerialNumber PSN, pid_t PID, char *Name) {
    return AXLibConstructApplication(PSN, PID, Name);
};
extern "C" void axlib_destroy_application(macos_application *Application) {
    AXLibDestroyApplication(Application);
};
// bool axlib_add_application_observer(macos_application *Application, ObserverCallback Callback) {
//     AXLibAddApplicationObserver(Application, Callback);
// };

struct macos_application_arr {
    size_t len;
    macos_application **arr;
};

extern "C" macos_application_arr axlib_running_processes(uint32_t ProcessFlags) {
    macos_application_arr Arr;
    std::vector<macos_application *> RunningProcesses = AXLibRunningProcesses(ProcessFlags);
    Arr.len = RunningProcesses.size();
    Arr.arr = &RunningProcesses[0];
    return Arr;
};
