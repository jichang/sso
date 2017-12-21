import { Component, Input, OnInit } from '@angular/core';

@Component({
  selector: 'form-control-error',
  templateUrl: './form-control-error.component.html',
  styleUrls: ['./form-control-error.component.css']
})
export class FormControlErrorComponent implements OnInit {
  @Input() errKey: string;
  @Input() errMsg: string;
  hidden: boolean = true;

  constructor() { }

  ngOnInit() {
  }

  update(hidden) {
    this.hidden = hidden;
  }
}
