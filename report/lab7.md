# Lab 7

## 编程

考虑到基于直觉的 ``obtain_lock()`` 和 ``drop()`` 的实现很有可能出现数据竞争，这里首先从 spin crate 抄来了一些代码：

```rust
// ...
    fn obtain_lock(&self)
    {
        while self.lock.compare_and_swap(false, true, Ordering::Acquire) != false
        {
            // Wait until the lock looks unlocked before retrying
            while self.lock.load(Ordering::Relaxed)
            {
                cpu_relax();
            }
        }
    }
// ...
    fn drop(&mut self)
    {
        self.lock.store(false, Ordering::Release);
    }
```

然后把 ``cpu_relax()`` 替换为 ``sleep(1)``。如果使用 ``yield_now()``，则会出现 yield 后又被调度器马上切换回来进而造成无限循环的情况。

## 回答一

``MutexGuard`` 负责自动释放锁，并保证用户获得的引用的生命期与锁的存续期相同。

## 回答二

需要分离使当前线程沉睡和切换到另一线程的代码逻辑。若使用原来的 ``yield_now()``，则可能会导致当前线程在未来再也无法被自动切换回来。

## 回答三

这一瞬间的中断开启允许超时的沉睡线程在此时借助时钟中断被唤醒，若不增加这一部分则可能导致沉睡的线程将永远沉睡。

## 回答四

若不修改，则可能导致在哲学家就餐场景中出现死锁，因为这时允许了异步中断，在以下两行代码

```rust
        let left = self.forks[left].lock();
        let right = self.forks[right].lock();
```

之间可能会发生线程切换，导致当前哲学家可能无法在两边叉子都可用时连续地占用它们，进而导致死锁。
