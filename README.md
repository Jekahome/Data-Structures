## Зачем нужно знать структуры данных и чем они полезны.

Хорошо подобранная структура данных позволяет эффективно манипулировать этими данными. 
Эффективно манипулировать - это про минимум занимаемой памяти, максимум скорости и производительность, масштабируемость, определенная логика, поведение, удобно, гибко, понятно ...  

"Никлаус Вирт написал книгу "Алгоритмы + структуры данных = программы."

- Stack
- Queue/Deque
- Linked list
- Set
- Map (Hash Table)
- Binary search tree
- Trie
- Graph

## Stack
    
LIFO (last-in, first-out) "Последним вошел - первым вышел"

**Чем полезен Stack?**

Особенности поведения Stack. 

- Для реализации рекурсии.
- Корректность выражения со скобками.
- Перевернуть слово — поместите все буквы в стопку и вытащите их. Из-за порядка стека LIFO буквы будут расположены в обратном порядке.
- В компиляторах — компиляторы используют стек для вычисления значения выражений типа `2 + 4 / 5 * (7 - 9)` путем преобразования выражения в префиксную или постфиксную форму.
- В браузерах — кнопка «Назад» в браузере сохраняет в стопке все URL-адреса, которые вы посещали ранее. Каждый раз, когда вы посещаете новую страницу, она добавляется поверх стека. При нажатии кнопки «Назад» текущий URL-адрес удаляется из стека и открывается доступ к предыдущему URL-адресу.

Сложность алгоритма для операции `push` и `pop` занимают постоянное время, т. е. `O(1)`.

**Способы реализации `Stack`**:

- Стек через `Array`, фиксированного размера, поиск `O(n)`, доступ по индексу `O(1)`, удаление `O(n)`  
- Стек через `Vec`, безразмерный, поиск `O(n)`, доступ по индексу `O(1)`, удаление `O(n)`
- Стек через `Linked list`, безразмерный, поиск `O(n)`, для двунаправленного поиск `O(n/2)`, возможность разбивать/объединять

[Stack](https://www.programiz.com/dsa/stack)

[Bad Stack](https://rust-unofficial.github.io/too-many-lists/first.html)

[Ok Stack](https://rust-unofficial.github.io/too-many-lists/second.html)

[Persistent Stack](https://rust-unofficial.github.io/too-many-lists/third.html)

[Визуализация](https://www.cs.usfca.edu/~galles/visualization/StackArray.html) 

 
## Queue

FIFO (first-in, first-out) "Первым пришел - первым ушел»"

Сложность алгоритма для операции `push` занимают постоянное время, т. е. `O(1)`.
Затратная операции `pop` за `O(n)` если реализация через Vec придется сдвигать элементы. 

**Чем полезна Queue?**
 
- Queue необходима для решения низкоуровневых задач, таких как планирование заданий CPU,
- Для моделирования реальной очереди - например, запросов на техническую поддерку, которые необходимо обрабатывать последовательно.  
- Для управления потоками в многопоточных средах.
- Для генерации значений.
- Для создания буферов.

**Способы реализации `Queue`**:

- Queue через `Vec`
- Queue через `Linked list`

[VecDeque in Rust](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)

 
## Linked list

`Linked list` - содержит в каждом елементе данные и указатель на следующий елемент.
Вставка `O(1)`, удаление `O(1)`, поиск `O(n)`

**Чем полезен Linked list?**

- Вам нужно много разбивать или объединять большие списки. 
- Вы делаете потрясающую параллельную работу без блокировки.
- Для построения более сложных структур данных.
- Для реализации файловых систем.
- Для формирования хэш-таблиц.
- Для выделения памяти в динамических структурах данных.

В большинстве случаев 99% вам следует просто использовать Vec (стек массивов),и остальные 1% времени вам следует использовать VecDeque
`Linked list` прекрасная структура данных с несколькими замечательными вариантами использования, но те случаи использования являются исключительными, а не обычными.
В функциональном языке Односвязные списки — ваш основной инструмент управления потоком управления, но они действительно плохой способ хранить кучу данных и запрашивать их.

**Плюсы:**

- динамический размер
- операция вставки/удаления в начало быстрее чем у динамического массива т.е. вектора (у вектора удаление `O(n)` из-за перестановки элементов)
- Сила `Linked list` заключается в возможности разорвать цепочку и снова присоединиться к ней.
- `Linked list` решает проблему массивов фиксированной размерности тем что может динамически расширятся.
- `Linked list` решает проблему динамических массивов тем что нет необходимости сдигать элементы справа.

**Минусы:** 

- поиск/доступ по индексу за `O(n)` (в худшем случае), для двунаправленного `O(n/2)` из-за точки входа из головы или хвоста

**Разновидности**

- Однонаправленный, каждый узел хранит адрес или ссылку на следующий узел в списке и последний узел имеет следующий адрес или ссылку как NULL.

`1->2->3->4->NULL`

- Двунаправленный, две ссылки, связанные с каждым узлом, одним из опорных пунктов на следующий узел и один к предыдущему узлу.

`NULL<-1<->2<->3->NULL`

- Круговой, все узлы соединяются, образуя круг. В конце нет NULL. Циклический связанный список может быть одно-или двукратным циклическим связанным списком.

`1->2->3->1`

[Linked list](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/blob/master/Chapter04/src/skip_list.rs)

[Doubly Linked List](https://www.programiz.com/dsa/linked-list-types#doubly)

[Linked List in Rust](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)

[too-many-lists](https://github.com/rust-unofficial/too-many-lists)

## Set (Множество)

Данные хранятся группой, их нельзя структурировать и в некоторых случаях нельзя сортировать. 
Зато с ними можно работать как с классическими математическими множествами: объединять `union`, искать пересечения `intersection`, вычислять разность `difference` и смотреть, является ли одно множество подмножеством другого `is_superset`.

**Чем полезен Set?**

- Для поддержания множества уникальных элементов.
- Для хранения данных, которые не нужно сортировать.
- Для сравнения, объединения наборов данных и других операций с ними.

[Set](https://practicum.yandex.ru/blog/10-osnovnyh-struktur-dannyh/)

[HashSet in Rust](https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.intersection)

## Map (Hash Table)

Её ещё называют ассоциативным массивом или словарём. 
Данные здесь хранятся в паре «ключ/значение», причем каждый ключ уникален, а вот значения могут повторяться.
Вставка `O(1)`, удаление `O(1)`, поиск `O(1)`. Но есть один недостаток, `Hash Table` имеют накладные расходы по памяти, храня хеш ключа.
Функцию хеширования можно выбрать самую производительную, но это дает возможность для вреда атаки HashDoS.

**Чем полезен Hash Table**
 
- требуется поиск и вставка c постоянным временем `O(1)`
- криптографические приложения
- необходимы данные индексирования
- Для создания баз, в которых нужно хранить уникальное соответствие между двумя множествами значений. Их помещают в ключ, и структура проверяет, чтобы ключ не повторялся

**Эффективность хеширования зависит от:**

- Функции хеширования
- Размера хэш-таблицы
- Метода борьбы с коллизиями (метод цепочек и метод двойного хеширования)

[Hash Table](https://www.programiz.com/dsa/hash-table)

[HashMap in Rust](https://doc.rust-lang.org/std/collections/struct.HashMap.html)

[Хеш-таблица](https://neerc.ifmo.ru/wiki/index.php?title=%D0%A5%D0%B5%D1%88-%D1%82%D0%B0%D0%B1%D0%BB%D0%B8%D1%86%D0%B0)

## Tree Data Structure

**Почему древовидная структура данных?**

Другие структуры данных, такие как массивы, связанный список, стек и очередь, представляют собой линейные структуры данных, в которых данные хранятся последовательно. Для выполнения какой-либо операции с линейной структурой данных временная сложность увеличивается с увеличением размера данных. Но это неприемлемо в современном вычислительном мире.

Различные древовидные структуры данных обеспечивают более быстрый и простой доступ к данным, поскольку это нелинейная структура данных.

Двоичное дерево является упорядоченным ориентированным деревом.


**Node**

Node — это объект, который содержит ключ или значение и указатели на его дочерние узлы.

Последние узлы каждого пути называются листовыми узлами или внешними узлами, которые не содержат ссылки/указателя на дочерние узлы.

Node, имеющий хотя бы дочерний Node, называется внутренним узлом.

**Edge**

Или Ребро это связь между любыми двумя узлами.

**Root**

Это самый верхний Node дерева.

**Высота Node**

Высота Node — это количество ребер от Node до самого глубокого листа (т. е. самый длинный путь от Node до листового узла).

**Глубина Node**

Глубина Node — это количество ребер от корня до Node.

**Высота дерева**

Высота дерева — это высота корневого Node или глубина самого глубокого Node.

**Degree of a Node**

Degree of a Node — это общее количество ветвей этого узла.

**Forest**

Совокупность непересекающихся деревьев называется Forest.

**Виды Tree**

- Binary Tree
- AVL Tree
- B-Tree

 
**Применение**

- `Binary search tree (BST)` используются для быстрой проверки наличия элемента в наборе или нет.
- `Heap` — это разновидность дерева, которое используется для сортировки кучи.
- Модифицированная версия дерева под названием `Tries` используется в современных маршрутизаторах для хранения информации о маршрутизации.
- Большинство популярных баз данных используют `B-деревья` и `T-деревья`, которые являются вариантами древовидной структуры, для хранения своих данных.
- Компиляторы используют синтаксическое дерево/префиксное дерево `Trie` для проверки синтаксиса каждой написанной вами программы.

## B-Tree

В Rust `std::collections::BTreeMap` реализован на основе `B-Tree`

Основной недостаток В-деревьев состоит в отсутствии для них эффективных средств выборки данных (то есть метода обхода дерева), упорядоченных по свойству, отличному от выбранного ключа.

[B-Tree](https://ru.wikipedia.org/wiki/B-%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%BE)

## Binary Tree

**Типы двоичного дерева:**
- Полное двоичное дерево
- Идеальное двоичное дерево
- Дегенеративное или патологическое дерево
- Перекошенное двоичное дерево
- Сбалансированное двоичное дерево (АВЛ-дерево (AVL Tree), Красно-чёрное дерево, splay-деревья)
- Двоичное дерево поиска (Binary Search Tree)
- Двоичная куча (Binary Heap)
- Матричное дерево
- Дерево Фибоначчи
- Суффиксное дерево

**Идеальное двоичное дерево**

Идеальное двоичное дерево - это тип двоичного дерева, в котором каждый внутренний узел имеет ровно два дочерних узла, а все конечные узлы находятся на одном уровне.

**Сбалансированное двоичное дерево**

Это тип двоичного дерева, в котором разница между высотой левого и правого поддерева для каждого узла равна 0 или 1.

Операции с деревом работают за `O(h)`, где `h` — максимальная глубина дерева (глубина — расстояние от корня до вершины). В оптимальном случае, когда глубина всех листьев одинакова, в дереве будет `n=2^h` вершин. Значит, сложность операций в деревьях, близких к оптимуму будет `O(log(n))`. К сожалению, в худшем случае дерево может выродится и сложность операций будет как у списка.

Существуют способы реализовать дерево так, чтобы оптимальная глубина дерева сохранялась при любой последовательности операций. Такие деревья называют сбалансированными. К ним например относятся красно-черные деревья, AVL-деревья, splay-деревья, и т.д.

[Сбалансированное двоичное дерево](https://habr.com/ru/articles/65617/)

**Двоичное дерево поиска**

В информатике, бинарное дерево поиска (BST), также называемое упорядоченным или отсортированное двоичное дерево.

**Применение:**
- Приложения двоичного дерева
- Для легкого и быстрого доступа к данным
- В алгоритмах маршрутизатора
- Для реализации структуры данных кучи
- Синтаксическое дерево

## Красно-черное дерево (Red-black tree, RB tree)

Разновидность сбалансированного двоичного дерева.
Все основные операции с красно-черным деревом можно реализовать за `O(h)`, то есть `O(log n)`

Свойства:

- Красная вершина не может быть сыном красной вершины
- Черная глубина любого листа одинакова (черной глубиной называют количество черных вершин на пути из корня)
- Корень дерева черный

Для реализации этого вида сбаласированных деревьев, нужно в каждой вершине хранить дополнительно 1 бит информации (цвет). Иногда это вызывает большой overhead из-за выравнивания. В таких случаях предпочтительно использовать структуры без дополнительных требований к памяти.

Красно-черные деревья широко используются — реализация set/map в стандартных библиотеках, различные применения в ядре Linux (для организации очередей запросов, в ext3 etc.)

Красно-черные деревья тесно связаны с B-деревьями. Можно сказать, что они идентичны B-деревьям порядка 4 (или 2-3-4 деревьям).
Красно-черные деревья — одни из наиболее активно используемых на практике самобалансирующихся деревьев. Они так называются из-за наличия на узле дополнительного поля, в котором размещается цвет узла. В качестве стандартных цветов используют обычно красные и черные узлы, а сам цвет узла используется во время балансировки.

Так как красно-черные деревья самобалансирующиеся, то среднее и худшее время поиска тоже составляют `O(logN)`. А операции вставки и удаления узла могут потребовать поворот поддерева.

Помимо особенностей работы с листовыми узлами к свойствам красно-черного так же относят:

Корень `красно-черного дерева` черный

Две красные вершины не могут идти подряд ни на одном пути. Оба потомка каждого красного узла — черные

Для каждой вершины, в каждом исходящем из нее пути, одинаковое число черных вершин

Иногда при работе с узлами `красно-черного дерева` используют черную высоту — количество черных вершин на исходящих из нее путях, не включая саму исходную вершину.

Чтобы вставить узел, мы сначала ищем в дереве место, куда его следует добавить. Новый узел всегда добавляется как лист, поэтому оба его потомка являются пустыми узлами и предполагаются черными. После вставки красим узел в красный цвет. Далее смотрим на предка и проверяем, не нарушается ли свойства дерева, которые описали выше. Если необходимо, мы перекрашиваем узел и производим поворот, чтобы сбалансировать дерево.

Сбалансированность этих деревьев хуже, чем у `АВЛ-деревьев`. При этом затраты на поддержание состояния сбалансированности и потребление памяти меньше, а операцию поиска можно выполнять одновременно с выполнением перестроения дерева.

Благодаря этим преимуществам сфера применения красно-черных деревьев существенно шире. Так, например, в стандартной библиотеке шаблонов языка C++ STL и TreeMap в Java применяются именно красно-черные деревья.

Сбалансированность `красно-черного дерева` деревьев хуже, чем у `АВЛ-деревьев`. При этом затраты на поддержание состояния сбалансированности и потребление памяти меньше, а операцию поиска можно выполнять одновременно с выполнением перестроения дерева.

`
class RBTreeNode {
  constructor(value, parent) {
    this.left = null; //ссылка на левый дочерний узел
    this.right = null; //ссылка на правый дочерний
    this.isRed = false; //цвет узла. Если не красный, то считаем что узел черный
    this.parent = parent; //ссылка на родителя
    this.value = value; //полезная нагрузка
  }
}
`
[Красно-черное дерево](https://habr.com/ru/articles/66926/)

[Красно-черные деревья](https://ru.hexlet.io/courses/algorithms-trees/lessons/balancing/theory_unit)

[Красно-черные деревья визуализация](https://people.ksp.sk/~kuko/gnarley-trees/Redblack.html)

[Red-Black Tree](https://www.programiz.com/dsa/red-black-tree)

## АВЛ-дерево (AVL Tree)

Накладывает на дерево следующее ограничение: у любой вершины высоты левого и правого поддеревьев должны отличаться не более чем на 1.

Реализация, как и у красно-черного дерева, основана на разборе случаев и достаточно сложна для понимания (хотя имхо проще красно-черного) и имеет сложность `O(log(n))` на все основные операции. Для работы необходимо хранить в каждой вершине разницу между высотами левого и правого поддеревьев. Так как она не превосходит 1, достаточно использовать 2 бита на вершину.

АВЛ-деревья отличаются от идеально сбалансированных. АВЛ-дерево считается сбалансированным, если для каждого узла дерева высота его правого и левого поддеревьев отличаются не более чем на единицу. Если модификация структуры узлов приводит к нарушению сбалансированности дерева, то необходимо выполнить его балансировку.

[АВЛ-дерево](https://habr.com/ru/articles/66926/)

[АВЛ-деревья](https://ru.hexlet.io/courses/algorithms-trees/lessons/balancing/theory_unit)

## Splay-дерево

Не накладывает никаких ограничений на структуру дерева. Более того, в процессе работы дерево может оказаться полностью разбалансированным!

Основа splay-дерева — операция splay. Она находит нужную вершину (или ближайшую к ней при отсутствии) и «вытягивает» ее в корень особой последовательностью элементарных вращений (локальная операция над деревом, сохраняющая свойство порядка, но меняющая структуру). Через нее можно легко выразить все оснавные операции с деревом. Последовательность операций в splay подобрана так, чтобы дерево «магически» работало быстро.

Зная магию операции splay, эти деревья реализуются не легко, а очень легко, поэтому они тоже очень популярны в ACM ICPC, Topcoder etc.

Ясно, что в таком дереве нельзя гарантировать сложность операций O(log(n)) (вдруг нас попросят найти глубоко залегшую вершину в несбалансированном на данный момент дереве?). Вместо этого, гарантирается амортизированная сложность операции O(log(n)), то есть любая последовательность из m операций с деревом размера n работает за O((n+m)*log(n)). Более того, splay-дерево обладает некоторыми магическими свойствами, за счет которого оно на практике может оказаться намного эффективнее остальных вариантов. Например, вершины, к которым обращались недавно, оказываются ближе к корню и доступ к ним ускоряется. Более того, доказано что если вероятности обращения к элементам фиксированы, то splay-дерево будет работать асимптотически не медленней любой другой реализации бинарных деревьев. Еще одно преимущество в том, что отсутствует overhead по памяти, так как не нужно хранить никакой дополнительной информации.

[Splay-дерево](https://habr.com/ru/articles/66926/)

## Binary search tree (Двоичное дерево поиска)

Дерево — это структура, данные в которой данные находятся в Nodes. У каждого Node могут быть один или несколько дочерних и только один родитель.
Вставка `O(LogN)`, удаление `O(LogN)`, поиск `O(LogN)`

**Свойства дерева поиска:**

- оба поддерева — левое и правое — являются двоичными деревьями поиска;
- у всех узлов левого поддерева произвольного узла X значения ключей данных меньше либо равны, нежели значение ключа данных самого узла X;
- у всех узлов правого поддерева произвольного узла X значения ключей данных больше, нежели значение ключа данных самого узла X.

**Чем полезно Binary search tree**

- Для быстрого поиска данных.
- Для хранения данных в отсортированном виде с возможностью быстро их добавлять и удалять.

Деревья двоичного поиска позволяют иметь двоичный поиск для быстрого поиска, добавления и удаления элементов данных и могут использоваться для реализации динамических наборов и таблиц поиска. 
Порядок узлов в BST означает, что каждое сравнение пропускает примерно половину оставшегося дерева, поэтому весь поиск занимает время, пропорциональное двоичному логарифму количества элементов, хранящихся в дереве. 
Это намного лучше, чем линейное время, необходимое для поиска элементов по ключу в (несортированном) массиве, но медленнее, чем соответствующие операции с хеш-таблицами. 

Основное преимущество двоичных деревьев поиска перед другими структурами данных состоит в том, что связанные алгоритмы сортировки и алгоритмы поиска, такие как обход по порядку, могут быть очень эффективными.

Двоичные деревья поиска - это фундаментальная структура данных, используемая для построения более абстрактных структур данных, таких как наборы, мультимножества и ассоциативные массивы.

**Способы обхода дерева**

(Поиск в глубину `Depth-First Search DFS` - На каждом шаге итератор пытается продвинуться вертикально вниз по дереву перед тем, как перейти к родственному узлу — узлу на том же уровне. Реализуется за счет использованием рекурсии, или с использованием стека)

Поиск в глубину (`DFS`) идет из начальной вершины, посещая еще не посещенные вершины без оглядки на удаленность от начальной вершины. Алгоритм поиска в глубину по своей природе является рекурсивным. Для эмуляции рекурсии в итеративном варианте алгоритма применяется структура данных стек.
`DFS` применяется в алгоритме нахождения компонентов сильной связности в ориентированном графе.
Для обхода в глубину существует рекурсивная реализация без стека, рекурсивная реализация со стеком и итеративная реализация со стеком.

- В симметричном порядке `In-order` (слева направо) — инфиксная форма, левое поддерево → корень → правое поддерево.

- В обратном порядке `Post-order` (снизу вверх) — постфиксная форма, левое поддерево → правое поддерево → корень.

- В прямом порядке `Pre-order` (сверху вниз) — префиксная форма, корень → левое поддерево → правое поддерево.

(Поиск в ширину Breadth-first — обход узлов дерева по уровням. Реализуется за счет использования очереди)
- В ширину: от корня и далее

Для бинарных деревьев поиска симметричный обход проходит все узлы в отсортированном порядке. Если мы хотим посетить узлы в обратно отсортированном порядке, то в коде рекурсивной функции симметричного обхода следует поменять местами правого и левого потомка.

Поиск в ширину (`Breadth-First Search BFS`) идет из начальной вершины, посещает сначала все вершины находящиеся на расстоянии одного ребра от начальной, потом посещает все вершины на расстоянии два ребра от начальной и так далее. Алгоритм поиска в ширину является по своей природе нерекурсивным (итеративным). Для его реализации применяется структура данных очередь (`FIFO`).
`BFS` применяется для нахождения кратчайшего пути в графе, в алгоритмах рассылки сообщений по сети, в сборщиках мусора, в программе индексирования – пауке поискового движка.

**Mетоды прохождения деревьев: рекурсия, очередь, стек**

**Рекурсивное** определение бинарного дерева определяет эту структуру как корень с двумя поддеревьями, которые идентифицируются полями левого и правого указателей в корневом узле. Сила рекурсии проявляется в методах прохождения деревьев. Каждый алгоритм прохождения дерева выполняет в узле три действия: заходит в узел и рекурсивно спускается по левому и правому поддеревьям. Спустившись к поддереву, алгоритм определяет, что он находится в узле, и может выполнить те же три действия. Спуск прекращается по достижении пустого дерева (указатель == NULL). Различные алгоритмы рекурсивного прохождения отличаются порядком, в котором они выполняют свои действия в узле. Мы изложим симметричный и обратный методы, в которых сначала осуществляется спуск по левому поддереву, а затем по правому. 
 
```
class BinaryTreeNode {
  traverseRecursive(node) {
    if (node != null) {
      console.log(`node = ${node.val}`);
      traverseRecursive(node.left);
      traverseRecursive(node.right);
    }
  }

  traverseWithStack() {
    let stack = [];
    stack.push(this);
    while (stack.length > 0) {
      let currentNode = stack.pop();

      console.log(`node = ${currentNode.val}`);
      if (currentNode.right != null) {
        stack.push(currentNode.right);
      }
      if (currentNode.left != null) {
        stack.push(currentNode.left);
      }
    }
  }

  traverseWithQueue() {
    let queue = [];
    queue.push(this.root);
    while (queue.length > 0) {
      let currentNode = queue.shift();
      console.log(`node = ${currentNode.val}`);
      if (currentNode.left) {
        queue.push(currentNode.left);
      }
      if (currentNode.right) {
        queue.push(currentNode.right);
      }
    }
  }
}
```

**Симметричный метод поиска в глубину**

Симметричный метод `In-order` прохождения начинает свои действия в узле спуском по его левому поддереву. Затем выполняется второе действие - обработка данных в узле. Третье действие - рекурсивное прохождение правого поддерева. В процессе рекурсивного спуска действия алгоритма повторяются в каждом новом узле.

`In-order` обход используется как раз когда нам надо проверять в начале детей и только потом подыматься к родительским узлам.

**Обратный метод поиска в глубину**

При `Post-order` обратном прохождении посещение узла откладывается до тех пор, пока не будут рекурсивно пройдены оба его поддерева. Порядок операций дает так называемое LRN (left, right, node) сканирование.

- Прохождение левого поддерева.
- Прохождение правого поддерева.
- Посещение узла.

`Post-order` обход используется когда нам нужно начать-так сказать с листов и завершить главным узлом — тоесть разложить дерево на то, как оно строилось.

**Прямой метод поиска в глубину** `Pre-order` определяется посещением узла в первую очередь и последующим прохождением сначала левого, а потом правого его поддеревьев.

`Pre-order` стоит использовать именно тогда, когда вы знаете что вам нужно проверить руты перед тем как проверять их листья.

```

                         _4_
                     _3_     _9_
                  _1_     _7_   10 
                     2   6   8

Прямой/Pre-order:      4  3  1  2  9  7  6   8  10
Симметричный/In-order: 1  2  3  4  6  7  8   9  10
Обратный/Post-order:   2  1  3  6  8  7  10  9  4
Breadth-First Search:  4  3  9  1  7  10 2   6  8
```

**Какой из методов использовать?**

- если вы знаете что решение где-то не далеко от вашей ноды — то лучше использовать обход в ширь, чтоб не закапываться глубоко в дерево
- если дерево очень глубокое, а решение редки — то лучше все таки попробовать поиск в ширь
- если дерево очень широкое, то можно попробовать поиск в глубь, потому как поиск в ширь может забрать слишком много времени.

### [Балансировка](https://ru.hexlet.io/courses/algorithms-trees/lessons/balancing/theory_unit)

Идеальная сбалансированность — это свойство дерева, при котором все его уровни, иногда кроме последнего, полностью заполнены.
Дерево с максимально возможной высотой (а) называется разбалансированным или вырожденным деревом. Оно не отличается от двусвязного списка, значит, теряет свое основное преимущество — скорость поиска.

Состояние идеальной сбалансированности в дереве трудно поддерживать. Любое добавление или удаление узла в сбалансированном дереве может привести к ситуации, когда дерево выйдет из идеально сбалансированного состояния. В таком случае скорость поиска будет значительно деградировать после каждой вставки или удаления узла.

Чтобы вернуть дерево в сбалансированный вид, его перестраивают после каждой манипуляции с составом узлов.
Стоит обратить внимание на `АВЛ-деревья` и `красно-черные деревья`, они являются максимально балансированными типами деревьев.

Ребалансировка деревьев осуществляется при помощи специальных механизмов — методов вращения. Вращения бывают двух видов: `левое и правое`.

**Вращение вправо выполняется за три шага:**

- Текущий корень поддерева (D) заменяется на левый дочерний узел (B)
- Предыдущий корень (D) становится правым дочерним узлом для (B)
- Предыдущее правое поддерево узла (B) становится левым поддеревом для (D)

**Вращение влево выполняется аналогично:**

- Текущий корень поддерева (D) заменяется на правый дочерний узел ©
- Предыдущий корень (D) становится левым дочерним узлом для ©
- Предыдущее левое поддерево узла © становится правым поддеревом для (D)


[Binary search tree](https://ru.wikipedia.org/wiki/%D0%94%D0%B2%D0%BE%D0%B8%D1%87%D0%BD%D0%BE%D0%B5_%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%BE_%D0%BF%D0%BE%D0%B8%D1%81%D0%BA%D0%B0)

[BST визуализация](https://people.ksp.sk/~kuko/gnarley-trees/BST.html)

[Способы обхода дерева](https://ru.hexlet.io/courses/algorithms-trees/lessons/binary/theory_unit)

[Способы обхода дерева](http://www.k-press.ru/cs/2000/3/trees/trees.asp)

[Способы обхода дерева](https://medium.com/@dimko1/%D0%B0%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC%D1%8B-%D0%BE%D0%B1%D1%85%D0%BE%D0%B4-%D0%B4%D0%B5%D1%80%D0%B5%D0%B2%D0%B0-ed54848c2d47)

[Trees](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-Rust/tree/master/Chapter05/src)

[Hands-On-Data-Structures-and-Algorithms-with-Rust](https://books.google.es/books?id=gYKFDwAAQBAJ&printsec=frontcover&redir_esc=y#v=onepage&q&f=false)

[Визуализация из текста](http://www.webgraphviz.com/?tab=map)

## [AST-деревья](https://ru.hexlet.io/courses/algorithms-trees/lessons/astree/theory_unit)

...


## Trie (Префиксное дерево, Бор, нагруженное дерево)

Данные в нём хранятся последовательно: каждый узел — это префикс, по которому находятся следующие узлы.

**Чем полезен Trie**

- Разновидность дерева для строк, быстрый поиск. Словари. Т9.
- Для хранения данных, которые нужно выдавать по цепочке. Например, слова для функции автозаполнения в телефоне: в зависимости от одной набранной буквы дерево выдаёт следующие.
- Для хранения данных, у которых есть повторяющиеся участки. Например IP-адресов.

[Префиксное дерево](https://ru.hexlet.io/courses/algorithms-trees/lessons/prefix/theory_unit)

## Graph

Граф — это более общий случай дерева. Иногда деревья называют ациклическими графами. 
По такому принципу устроены социальные сети: узлы — это люди, а рёбра — их отношения.

Storage `O(V+E)`, Add Vertex `O(1)`, Add Edge `O(1)`, Remove Vertex `O(V+E)`, Remove Edge `O(E)`, Query `O(V)`

Отличий у этих структур данных два:

- В графе возможны циклы, то есть «ребёнок» может быть «родителем» для того же элемента.
- Рёбра тоже могут нести смысловую нагрузку, то есть нужно сохранять их значения.

Графы бывают ориентированные и неориентированные. У ориентированных рёбра между узлами имеют направление, так что порядок элементов важен. У неориентированных направлений нет, и элементы можно читать и обходить в любом порядке.

**Чем полезен Graph**

- Для хранения информации, связанной друг с другом сложными соотношениями.
- Для анализа соотносящейся друг с другом информации.
- Для построения маршрута из точки А в точку Б.

**Встречаются в таких формах как**

- Матрица смежности
- Список смежности

Список смежности можно представить как перечень элементов, где слева находится один узел, а справа — все остальные узлы, с которыми он соединяется.

Матрица смежности — это сетка с числами, где каждый ряд или колонка соответствуют отдельному узлу в графе. На пересечении ряда и колонки находится число, которое указывает на наличие связи. Нули означают, что она отсутствует; единицы — что связь есть. Чтобы обозначить вес каждой связи, используют числа больше единицы.

**Общие алгоритмы обхода для просмотра рёбер и вершин графа**

- Поиск в ширину (breadth-first search) – обход по уровням
- Поиск в глубину (depth-first search) – обход по вершинам

[Graph Поиск в ширину](https://ru.algorithmica.org/cs/shortest-paths/bfs/)

```
Элементарные структуры данных
Хеширование и хеш-таблицы
Бинарные структуры поиска
Красно-черные деревья
Расширение структур данных
```