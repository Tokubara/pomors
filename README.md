# pomors
##### 功能上的区别

* 限定了番茄总数, 而不是无限循环(原项目是无限循环, 直到按q退出). 因此增加了总番茄数选项. 而且从指定秒数变成了指定分钟. 还增加了几个番茄对应一个长休息的选项. 具体增加的选项, 见Option的定义.
* 去掉了番茄和休息结束后的确认继续. 不再在终端打印询问信息, 直接继续.
* 时钟的显示信息, 多了总番茄数(因为原来没有这个逻辑).
* 所有番茄结束后增加通知信息`All done`
* 去掉声音播放
* 不每秒显示, 每10秒刷新时间的显示. 用sleep而不是spin_sleep

##### refactor

合并`flush_work_timer`和`flush_break_timer`为`flush_timer`

