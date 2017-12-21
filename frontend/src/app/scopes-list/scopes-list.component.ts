import { Component, Input, OnInit } from '@angular/core';
import { Scope } from '../scope-model.service';

@Component({
  selector: 'scopes-list',
  templateUrl: './scopes-list.component.html',
  styleUrls: ['./scopes-list.component.css']
})
export class ScopesListComponent implements OnInit {
  @Input() scopes: Scope[];
  constructor() { }

  ngOnInit() {
  }

}
