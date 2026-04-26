# [ComboBox](http://fox-toolkit.org/ref/classFX_1_1FXComboBox.html#details)

A Combo Box provides a way to select a string from a list of strings.

Unless COMBOBOX_STATIC is passed, it also allows the user to enter a new string into the text field, for example if the desired entry is not in the list of strings. Passing COMBOBOX_REPLACE, COMBOBOX_INSERT_BEFORE, COMBOBOX_INSERT_AFTER, COMBOBOX_INSERT_FIRST, or COMBOBOX_INSERT_LAST causes a newly entered text to replace the current one in the list, or be added before or after the current entry, or to be added at the beginning or end of the list. Combo Box is intended to enter text; if you need to enter a choice from a list of options, it is recommended that the List Box widget is used instead. When the text in the field is changed, a SEL_COMMAND will be send to the target. The Combo Box can also receive ID_GETSTRINGVALUE and ID_SETSTRINGVALUE and so on, which will behave similar to Text Field in that they will retrieve or update the value of the field.

# [ListBox](http://fox-toolkit.org/ref/classFX_1_1FXListBox.html#details)

The List Box is a control to select one of a list of options.

It looks similar to a Combo Box except that List Box yields integer numbers only. When an option is selected, List Box will send an SEL_COMMAND with the index of the opton. While manipulating the list, it may send SEL_CHANGED messages to indicate which option the cursor is hovering over. The List Box is able to receive ID_GETINTVALUE and ID_SETINTVALUE which will retrieve the current option or change the selected option. When items are added, replaced, or removed, the list sends messages of the type SEL_INSERTED, SEL_REPLACED, or SEL_DELETED, with the index of the affected item as argument.

# [List](http://fox-toolkit.org/ref/classFX_1_1FXList.html#details)
