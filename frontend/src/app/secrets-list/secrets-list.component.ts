import { Component, Input, OnInit, Output, EventEmitter } from "@angular/core";
import { Secret } from "../secret-model.service";

@Component({
  selector: "secrets-list",
  templateUrl: "./secrets-list.component.html",
  styleUrls: ["./secrets-list.component.css"]
})
export class SecretsListComponent implements OnInit {
  columns: string[] = ["client_id", "client_secret", "status", "action"];

  @Input() secrets: Secret[];
  @Output() remove = new EventEmitter();

  constructor() { }

  ngOnInit() { }

  removeSecret(secret: Secret) {
    this.remove.emit(secret);
  }
}
