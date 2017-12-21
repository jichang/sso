import {Directive, ElementRef, Renderer2, forwardRef} from '@angular/core';

import {ControlValueAccessor, NG_VALUE_ACCESSOR} from '@angular/forms';

export const CHECKBOX_VALUE_ACCESSOR: any = {
  provide: NG_VALUE_ACCESSOR,
  useExisting: forwardRef(() => DateControlValueAccessorDirective),
  multi: true,
};

@Directive({
  selector: 'input[type=date][formControlName],input[type=date][formControl],input[type=date][ngModel]',
  host: {'(change)': 'onChange($event.target.valueAsDate)', '(blur)': 'onTouched()'},
  providers: [CHECKBOX_VALUE_ACCESSOR]
})
export class DateControlValueAccessorDirective implements ControlValueAccessor {
  onChange = (_: any) => {};
  onTouched = () => {};

  constructor(private _renderer: Renderer2, private _elementRef: ElementRef) {}

  writeValue(value: any): void {
    this._renderer.setProperty(this._elementRef.nativeElement, 'valueAsDate', value);
  }
  registerOnChange(fn: (_: any) => {}): void { this.onChange = fn; }
  registerOnTouched(fn: () => {}): void { this.onTouched = fn; }

  setDisabledState(isDisabled: boolean): void {
    this._renderer.setProperty(this._elementRef.nativeElement, 'disabled', isDisabled);
  }
}