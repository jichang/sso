import { Component, Input, OnInit, Output, EventEmitter } from "@angular/core";
import { Scope } from "../scope-model.service";

@Component({
  selector: "scopes-list",
  templateUrl: "./scopes-list.component.html",
  styleUrls: ["./scopes-list.component.css"]
})
export class ScopesListComponent implements OnInit {
  columns: string[] = ["name", "description", "status", "action"];

  @Input() scopes: Scope[];
  @Output() remove = new EventEmitter();

  constructor() {}

  ngOnInit() {}

  removeScope(scope: Scope) {
    this.remove.emit(scope);
  }
}
