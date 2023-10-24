### lab1 实现的功能

- 根据作业的要求说明，给`TCB` 添加了几个必要的参数:
    - `is_running` 标记任务是否位`running`状态，因为每个任务都有可能中断而被其他任务抢占
    - `start_time` 记录任务的启动时刻，因为`taskInfo` 里面的`time` 是该任务的运行时常，因此要计算运行时常，应该在调用查询`taskInfo`的时刻 - `start_time`
    - `syscall_cnt` 记录任务被调用的次数，根据说明调用不超过500，由此设置一个根据不同`sys`调用函数类的id为下标的`vec`来统计当前`task`的被调用次数
- 修改`taskManager`
    - 对于`tasks`中的每个`task`,在初始化的时候,`isRunning`为`false`, `start_time`应该为0,且`syscall_cnt`为一个容量是500，初始值为`0`的`vec`.
    - 同时，要添加`update_syscall_cnt`的函数，及时更新每个`task`被`syscall`相应函数调用的次数；以及`get_taskInfo`函数来获取当前`task`



### 简答题

1. 

    1. `ch2b_bad_address`

    2. `ch2b_bad_instructions`

        

    3. `ch2b_bad_register`

