package com.plugin.txmap

import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData
import androidx.lifecycle.ViewModel
import com.plugin.txmap.com.plugin.txmap.args.Task
import com.plugin.txmap.com.plugin.txmap.args.TxMapOptions
import java.util.HashMap

class TaskViewModel : ViewModel() {
    // 使用私有MutableLiveData和公有LiveData来保护数据
    private val _sharedData = MutableLiveData<TxMapOptions>()
    val sharedData: LiveData<TxMapOptions> = _sharedData

    // 更新数据的方法
    fun updateData(newValue: TxMapOptions) {
        _sharedData.value = newValue
    }

    fun addTask(task: Task) {
        val current = _sharedData.value ?: TxMapOptions()
        current.tasks?.set(task.id, task)
        _sharedData.postValue(current)
    }
}