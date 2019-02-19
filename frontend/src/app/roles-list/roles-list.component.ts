import { Component, Input, Output, EventEmitter } from "@angular/core";
import { Role } from "../role-model.service";
import { Permission } from "../permission-model.service";
import { MatSlideToggleChange } from "@angular/material";

@Component({
  selector: "roles-list",
  templateUrl: "./roles-list.component.html",
  styleUrls: ["./roles-list.component.css"]
})
export class RolesListComponent {
  columns: string[] = ["resource_type", "action_type", "toggle"];
  @Input() roles: Role[];
  @Input() permissions: Permission[];
  @Output() grantPermission = new EventEmitter();
  @Output() revokePermission = new EventEmitter();

  isChecked(role: Role, permission: Permission) {
    return (
      role.permissions.findIndex(
        _permission =>
          _permission.resource_type === permission.resource_type &&
          _permission.action_type === permission.action_type
      ) !== -1
    );
  }

  updatePermission(
    event: MatSlideToggleChange,
    role: Role,
    permission: Permission
  ) {
    if (event.checked) {
      this.grantPermission.emit({
        role,
        permission
      });
    } else {
      this.revokePermission.emit({
        role,
        permission
      });
    }
  }
}
