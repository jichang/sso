import { Component, Input, OnInit, ViewChild, ElementRef } from '@angular/core';

@Component({
  selector: 'copy-text-span',
  templateUrl: './copy-text-span.component.html',
  styleUrls: ['./copy-text-span.component.css']
})
export class CopyTextSpanComponent implements OnInit {
  @Input() text: string;
  @ViewChild('textSpan', { static: true }) spanChild: ElementRef;

  constructor() { }

  ngOnInit() {
  }

  copy() {
    var range = document.createRange();
    range.selectNodeContents(this.spanChild.nativeElement);
    var selection = window.getSelection();
    selection.removeAllRanges();
    selection.addRange(range);

    try {
      document.execCommand("copy");
    } catch (e) {
      console.log(e);
    }
  }
}
